use llm::{LoadProgress, Model, ModelArchitecture};
use spinoff::{spinners::Dots2, Spinner};
use std::io::{copy, Read, Write};
use std::{error::Error, path::PathBuf, time::Instant};

fn load_progress_callback(
    mut sp: Option<Spinner>,
    now: Instant,
    mut prev_load_time: Instant,
) -> impl FnMut(LoadProgress) {
    move |progress| match progress {
        LoadProgress::HyperparametersLoaded => {
            if let Some(sp) = sp.as_mut() {
                sp.update_text("Loaded hyperparameters")
            };
        }
        LoadProgress::TensorLoaded {
            current_tensor,
            tensor_count,
            ..
        } => {
            if prev_load_time.elapsed().as_millis() > 500 {
                if let Some(sp) = sp.as_mut() {
                    sp.update_text(format!(
                        "Loaded tensor {}/{}",
                        current_tensor + 1,
                        tensor_count
                    ));
                };
                prev_load_time = std::time::Instant::now();
            }
        }
        LoadProgress::LoraApplied { name, source } => {
            if let Some(sp) = sp.as_mut() {
                sp.update_text(format!(
                    "Applied LoRA: {} from '{}'",
                    name,
                    source.file_name().unwrap().to_str().unwrap()
                ));
            };
        }
        LoadProgress::Loaded {
            file_size,
            tensor_count,
        } => {
            if let Some(sp) = sp.take() {
                sp.success(&format!(
                    "Loaded {tensor_count} tensors after {}ms",
                    now.elapsed().as_millis()
                ));
            };
        }
        _ => {}
    }
}

pub fn download() -> Box<dyn Model> {
    let url = "https://huggingface.co/rustformers/mpt-7b-ggml/resolve/main/mpt-7b-q4_0.bin";
    let archetecture = ModelArchitecture::Mpt;
    let context_size = 2024;

    let path = download_model(url).unwrap();

    let sp = Some(Spinner::new(Dots2, "Loading model...", None));

    let now = Instant::now();
    let prev_load_time = now;

    let model_params = llm::ModelParameters {
        prefer_mmap: true,
        context_size,
        lora_adapters: None,
        use_gpu: true,
    };

    llm::load_dynamic(
        archetecture,
        &path,
        llm::TokenizerSource::Embedded,
        model_params,
        load_progress_callback(sp, now, prev_load_time),
    )
    .unwrap_or_else(|err| panic!("Failed to load model from {path:?}: {err}"))
}

fn download_model(model_url: &str) -> Result<PathBuf, Box<dyn Error>> {
    let path: PathBuf = format!("./{}", model_url.rsplit_once('/').unwrap().1).into();
    if path.exists() {
        return Ok(path);
    }
    let response = reqwest::blocking::get(model_url)?;
    println!("downloading model. This will take several minutes");

    let mut file = std::fs::File::create(&path)?;

    let size = response.content_length().unwrap_or(4_294_967_296) as usize;

    let mut stream = response.bytes()?;
    copy(&mut &*stream, &mut file)?;

    file.flush()?;

    println!("Finished Downloading");

    Ok(path)
}
