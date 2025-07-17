// use include_dir::DirEntry;
// use model::Project;
// use once_cell::sync::Lazy;

// static EXAMPLES: include_dir::Dir = include_dir::include_dir!("$CARGO_MANIFEST_DIR/examples");

// pub fn get_welcome_project() -> Project {
//     get_example_projects()
//         .iter()
//         .find(|p| &p.path == "welcome.rs")
//         .unwrap()
//         .clone()
// }

// /// Returns a list of all example projects.
// pub fn get_example_projects() -> &'static [Project] {
//     static LIST: Lazy<Vec<Project>> = once_cell::sync::Lazy::new(|| {
//         let mut projects = Vec::new();

//         for entry in EXAMPLES.entries() {
//             let DirEntry::File(entry) = entry else {
//                 continue;
//             };

//             let path = entry.path();
//             let contents = entry.contents();
//             let contents = String::from_utf8(contents.to_vec()).unwrap();

//             let mut description = String::new();

//             for line in contents.lines() {
//                 if let Some(line) = line.strip_prefix("//!") {
//                     description.push_str(line);
//                     description.push('\n');
//                 } else {
//                     break;
//                 }
//             }

//             // Remove the trailing newline
//             description.pop();

//             let mut project = Project::new(
//                 contents,
//                 Some(description),
//                 Some(path.to_string_lossy().to_string()),
//             );

//             project.prebuilt = true;

//             projects.push(project);
//         }

//         projects
//     });

//     LIST.as_ref()
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn has_projects() {
//         assert!(!dbg!(get_example_projects()).is_empty());
//     }

//     #[test]
//     fn has_welcome() {
//         dbg!(get_welcome_project());
//     }
// }
