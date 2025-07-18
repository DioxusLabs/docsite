use anyhow::{Context, Result};
use clap::Parser;
use pulldown_cmark::{Event, Tag, TagEnd};
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input folder containing Notion markdown export
    #[arg(short, long)]
    input: PathBuf,

    /// Output folder for transformed markdown
    #[arg(short, long)]
    output: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if !args.input.exists() {
        anyhow::bail!("Input folder does not exist: {}", args.input.display());
    }

    // Create output directory if it doesn't exist
    fs::create_dir_all(&args.output).with_context(|| {
        format!(
            "Failed to create output directory: {}",
            args.output.display()
        )
    })?;

    // Process all markdown files in the input directory
    for entry in WalkDir::new(&args.input) {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("md") {
            println!("Processing: {}", path.display());
            process_markdown_file(&args.input, &args.output, path).await?;
        }
    }

    println!("✅ Transformation complete!");
    Ok(())
}

async fn process_markdown_file(
    input_base: &Path,
    output_base: &Path,
    md_path: &Path,
) -> Result<()> {
    // Read the markdown file
    let content = fs::read_to_string(md_path)
        .with_context(|| format!("Failed to read file: {}", md_path.display()))?;

    // Get the relative path from input base to maintain directory structure
    let relative_path = md_path.strip_prefix(input_base)?;
    let output_md_path = output_base.join(relative_path);

    // Create parent directories if needed
    if let Some(parent) = output_md_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Create corresponding assets folder
    let assets_folder_name = md_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("assets");
    let input_assets_folder = md_path.parent().unwrap().join(assets_folder_name);
    let output_assets_folder = output_md_path.parent().unwrap().join("assets");

    // Process images if assets folder exists
    let mut image_mapping = HashMap::new();
    if input_assets_folder.exists() {
        fs::create_dir_all(&output_assets_folder)?;
        image_mapping = process_images(&input_assets_folder, &output_assets_folder).await?;
    }

    // Transform the markdown content
    let transformed_content = transform_markdown(&content, &image_mapping)?;

    // Write the transformed markdown
    fs::write(&output_md_path, transformed_content)
        .with_context(|| format!("Failed to write file: {}", output_md_path.display()))?;

    println!("  ✓ Processed: {}", relative_path.display());
    Ok(())
}

async fn process_images(
    input_folder: &Path,
    output_folder: &Path,
) -> Result<HashMap<String, String>> {
    let mut image_mapping = HashMap::new();
    let mut screenshot_counter = 1;

    for entry in WalkDir::new(input_folder) {
        let entry = entry?;
        let path = entry.path();

        // Skip directories
        if path.is_dir() {
            continue;
        }

        if let Some(extension) = path.extension().and_then(|s| s.to_str()) {
            let file_name = path.file_name().unwrap().to_str().unwrap();

            if matches!(
                extension.to_lowercase().as_str(),
                "png" | "jpg" | "jpeg" | "gif" | "webp"
            ) {
                // Convert images to AVIF
                let new_name = generate_new_image_name(file_name, &mut screenshot_counter);
                let new_name_avif = format!(
                    "{}.avif",
                    new_name.trim_end_matches(&format!(".{}", extension))
                );

                // Convert to AVIF
                let img = image::open(path)
                    .with_context(|| format!("Failed to open image: {}", path.display()))?;

                let output_path = output_folder.join(&new_name_avif);
                img.save_with_format(&output_path, image::ImageFormat::Avif)
                    .with_context(|| format!("Failed to save AVIF: {}", output_path.display()))?;

                // Store mapping from original to new name (without ./assets/ prefix)
                image_mapping.insert(file_name.to_string(), new_name_avif.clone());
                println!("    ✓ Converted: {} -> {}", file_name, new_name_avif);
            } else {
                // Copy all other assets (videos, documents, etc.) as-is
                let cleaned_name = clean_asset_name(file_name);
                let output_path = output_folder.join(&cleaned_name);

                fs::copy(path, &output_path).with_context(|| {
                    format!(
                        "Failed to copy asset: {} -> {}",
                        path.display(),
                        output_path.display()
                    )
                })?;

                // Store mapping from original to cleaned name (without ./assets/ prefix)
                image_mapping.insert(file_name.to_string(), cleaned_name.clone());
                println!("    ✓ Copied: {} -> {}", file_name, cleaned_name);
            }
        }
    }

    Ok(image_mapping)
}

fn clean_asset_name(original_name: &str) -> String {
    // Clean up asset names by removing spaces and URL encoding
    original_name
        .replace(" ", "-")
        .replace("%20", "-")
        .to_lowercase()
}

fn generate_new_image_name(original_name: &str, screenshot_counter: &mut i32) -> String {
    let lower_name = original_name.to_lowercase();

    // Check if it's a screenshot
    let screenshot_regex = Regex::new(r"screenshot[_\s]*\d{4}[-_]\d{2}[-_]\d{2}").unwrap();
    if screenshot_regex.is_match(&lower_name) {
        let name = format!("screenshot-{}", screenshot_counter);
        *screenshot_counter += 1;
        return name;
    }

    // For other images, just clean up the name
    let cleaned = original_name
        .replace(" ", "-")
        .replace("%20", "-")
        .to_lowercase();

    // Remove file extension to add it back later
    if let Some(dot_pos) = cleaned.rfind('.') {
        cleaned[..dot_pos].to_string()
    } else {
        cleaned
    }
}

fn transform_markdown(content: &str, image_mapping: &HashMap<String, String>) -> Result<String> {
    let parser = pulldown_cmark::Parser::new(content);
    let mut events = Vec::new();
    let mut skip_until_after_heading = false;
    let mut in_link = false;

    for event in parser {
        match event {
            Event::Start(Tag::Heading { .. }) => {
                events.push(event);
            }
            Event::End(TagEnd::Heading(_)) => {
                skip_until_after_heading = true;
                events.push(event);
            }
            Event::Rule if skip_until_after_heading => {
                // Skip the horizontal rule after heading
                skip_until_after_heading = false;
                continue;
            }
            Event::Start(Tag::Link {
                link_type,
                dest_url,
                title,
                id,
            }) => {
                let processed_url = process_image_url(&dest_url, image_mapping);

                // Check if this is a video file - if so, convert to image syntax
                let url_decoded = dest_url.replace("%20", " ");
                if let Some(filename) = Path::new(&url_decoded).file_name().and_then(|s| s.to_str())
                {
                    if is_media_file(filename) {
                        // Convert link to image for video files
                        events.push(Event::Start(Tag::Image {
                            link_type,
                            dest_url: processed_url.into(),
                            title,
                            id,
                        }));
                        continue;
                    }
                }

                // Regular link handling
                in_link = true;
                events.push(Event::Start(Tag::Link {
                    link_type,
                    dest_url: processed_url.into(),
                    title,
                    id,
                }));
            }
            Event::End(TagEnd::Link) => {
                if in_link {
                    in_link = false;
                    events.push(event);
                } else {
                    // This might be a link that was converted to an image
                    // Convert the end event to an image end event
                    events.push(Event::End(TagEnd::Image));
                }
            }
            Event::Text(text) => {
                if in_link {
                    // Don't process text inside links
                    events.push(Event::Text(text));
                } else {
                    // Process image references in text only if not inside a link
                    let processed_text = process_image_references(&text, image_mapping);
                    events.push(Event::Text(processed_text.into()));
                }
            }
            Event::Start(Tag::Image {
                link_type,
                dest_url,
                title,
                id,
            }) => {
                let processed_url = process_image_url(&dest_url, image_mapping);
                events.push(Event::Start(Tag::Image {
                    link_type,
                    dest_url: processed_url.into(),
                    title,
                    id,
                }));
            }
            _ => {
                events.push(event);
            }
        }
    }

    // Convert events back to markdown using a simple approach
    let mut output = String::new();
    let mut in_paragraph = false;
    let mut list_stack: Vec<bool> = Vec::new(); // true for ordered, false for unordered
    let mut in_image = false;
    let mut current_link_url: Option<String> = None;

    for event in events {
        match event {
            Event::Start(Tag::Paragraph) => {
                if !output.is_empty() && !output.ends_with('\n') {
                    output.push('\n');
                }
                in_paragraph = true;
            }
            Event::End(TagEnd::Paragraph) => {
                if in_paragraph {
                    output.push_str("\n\n");
                    in_paragraph = false;
                }
            }
            Event::Start(Tag::Heading { level, .. }) => {
                if !output.is_empty() && !output.ends_with('\n') {
                    output.push('\n');
                }
                output.push_str(&"#".repeat(level as usize));
                output.push(' ');
            }
            Event::End(TagEnd::Heading(_)) => {
                output.push_str("\n\n");
            }
            Event::Start(Tag::List(start_num)) => {
                list_stack.push(start_num.is_some());
                if !output.ends_with('\n') {
                    output.push('\n');
                }
            }
            Event::End(TagEnd::List(_)) => {
                list_stack.pop();
                output.push('\n');
            }
            Event::Start(Tag::Item) => {
                let is_ordered = list_stack.last().copied().unwrap_or(false);
                if is_ordered {
                    output.push_str("1. ");
                } else {
                    output.push_str("- ");
                }
            }
            Event::End(TagEnd::Item) => {
                output.push('\n');
            }
            Event::Start(Tag::Image {
                dest_url, title, ..
            }) => {
                output.push_str("![");
                if !title.is_empty() {
                    output.push_str(&title);
                }
                output.push_str("](");
                output.push_str(&dest_url);
                output.push(')');
                in_image = true;
            }
            Event::End(TagEnd::Image) => {
                in_image = false;
            }
            Event::Start(Tag::Link { dest_url, .. }) => {
                output.push('[');
                current_link_url = Some(dest_url.to_string());
            }
            Event::End(TagEnd::Link) => {
                output.push(']');
                if let Some(url) = current_link_url.take() {
                    output.push('(');
                    output.push_str(&url);
                    output.push(')');
                }
            }
            Event::Start(Tag::Strong) => {
                output.push_str("**");
            }
            Event::End(TagEnd::Strong) => {
                output.push_str("**");
            }
            Event::Start(Tag::Emphasis) => {
                output.push('*');
            }
            Event::End(TagEnd::Emphasis) => {
                output.push('*');
            }
            Event::Start(Tag::CodeBlock(kind)) => {
                output.push_str("```");
                if let pulldown_cmark::CodeBlockKind::Fenced(lang) = kind {
                    output.push_str(&lang);
                }
                output.push('\n');
            }
            Event::End(TagEnd::CodeBlock) => {
                output.push_str("```\n\n");
            }
            Event::Code(text) => {
                output.push('`');
                output.push_str(&text);
                output.push('`');
            }
            Event::Text(text) if !in_image => {
                output.push_str(&text);
            }
            Event::Text(_) if in_image => {
                // Skip text inside images - it's already handled in the alt text
            }
            Event::SoftBreak => {
                output.push('\n');
            }
            Event::HardBreak => {
                output.push_str("  \n");
            }
            Event::Rule => {
                output.push_str("\n---\n\n");
            }
            _ => {
                // Handle other events as needed
            }
        }
    }

    Ok(output)
}

fn process_image_references(text: &str, image_mapping: &HashMap<String, String>) -> String {
    let mut result = text.to_string();

    // Only process if this text doesn't look like it's already part of a processed link
    if result.contains("./assets/") {
        return result;
    }

    // Replace image and video references
    for (original, new) in image_mapping {
        // Handle URL-encoded spaces and direct references
        let encoded_original = original.replace(" ", "%20");
        let new_with_prefix = format!("./assets/{}", new);

        // Check if this is a video file that should be treated as an image
        let is_video = is_media_file(original);

        // Create replacements for both link and image syntax
        let old_link = format!("[{}]", original);
        let old_link_encoded = format!("[{}]", encoded_original);

        if is_video {
            // Convert video links to image syntax: [video.mp4] -> ![video.mp4](./assets/video.mp4)
            let new_image = format!("![{}]({})", original, new_with_prefix);
            let new_image_encoded = format!("![{}]({})", encoded_original, new_with_prefix);

            result = result.replace(&old_link, &new_image);
            result = result.replace(&old_link_encoded, &new_image_encoded);
        } else {
            // For non-video files, just update the path
            result = result.replace(original, &new_with_prefix);
            result = result.replace(&encoded_original, &new_with_prefix);
        }
    }

    result
}

fn process_image_url(url: &str, image_mapping: &HashMap<String, String>) -> String {
    // Extract filename from URL
    let url_decoded = url.replace("%20", " ");

    if let Some(filename) = Path::new(&url_decoded).file_name().and_then(|s| s.to_str()) {
        if let Some(new_name) = image_mapping.get(filename) {
            return format!("./assets/{}", new_name);
        }
    }

    // Fallback: clean up the URL by removing URL encoding
    format!("./assets/{}", url.replace("%20", "-").to_lowercase())
}

fn is_media_file(filename: &str) -> bool {
    if let Some(extension) = Path::new(filename).extension().and_then(|s| s.to_str()) {
        matches!(
            extension.to_lowercase().as_str(),
            "mp4"
                | "mov"
                | "avi"
                | "mkv"
                | "webm"
                | "m4v"
                | "3gp"
                | "png"
                | "jpg"
                | "jpeg"
                | "gif"
                | "webp"
                | "avif"
        )
    } else {
        false
    }
}
