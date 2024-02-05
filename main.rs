use std::{env, path::Path};

use walkdir::{DirEntry, WalkDir};

use crate::logger::Logger;

mod logger;
mod visitor;

// Instructions:
// create a `test.js`,
// run `cargo run -p js-visitor -- --ignore-dynamic-error`

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let mut files = vec![];

    let mut panic_on_dynamic_errors = true;

    // simple argument parsing
    for arg in args.iter() {
        let positional = !arg.starts_with('-');

        if positional {
            files.push(arg.to_string())
        }

        match arg.as_str() {
            "-i" | "--ignore-dynamic-error" => panic_on_dynamic_errors = false,
            _ => (),
        }
    }

    if files.is_empty() {
        panic!("No files provided")
    }

    let mut package_names = vec![];

    Logger::info("Starting to process files and directories");

    for file in &files {
        Logger::info(&format!("Processing file: {}", file));
        let path = std::path::Path::new(&file);

        match path {
            _ if path.is_file() => {
                if !should_process_file(path) {
                    Logger::info(&format!("Skipping file: {}", file));
                    continue;
                }
                let mut package_names_for_file =
                    visitor::extract_imports(path, panic_on_dynamic_errors);
                package_names.append(&mut package_names_for_file);
            }
            _ if path.is_dir() => {
                Logger::info(&format!("Processing directory: {}", file));

                let walker = WalkDir::new(file).into_iter();
                for entry in walker.filter_entry(|e| !is_ignored_entry(e)) {
                    let entry = entry.unwrap();
                    let entry_path = entry.path();

                    // don't process directories
                    if entry_path.is_dir() {
                        Logger::info(&format!(
                            "Diving into directory: {}",
                            entry_path.to_str().unwrap()
                        ));
                        continue;
                    }

                    // only process known files
                    if should_process_file(entry_path) && entry_path.is_file() {
                        let mut package_names_for_file =
                            visitor::extract_imports(entry_path, panic_on_dynamic_errors);
                        package_names.append(&mut package_names_for_file);
                    }
                }
            }
            _ => {}
        }
    }

    package_names.dedup();

    // print list to stdout
    if package_names.is_empty() {
        Logger::info("No packages found")
    } else {
        Logger::info("Found packages:");
        for package_name in package_names {
            println!("- {}", package_name);
        }
    }
}

const VALID_FILE_EXTENSIONS: [&str; 4] = ["js", "jsx", "ts", "tsx"];
fn should_process_file(path: &Path) -> bool {
    VALID_FILE_EXTENSIONS
        .iter()
        .any(|valid_ext| match path.is_file() {
            true => path
                .extension()
                .and_then(std::ffi::OsStr::to_str)
                .map(|ext| ext.eq(*valid_ext))
                .unwrap_or(false),
            false => false,
        })
}

fn is_ignored_entry(entry: &DirEntry) -> bool {
    let is_dot = entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false);

    let path = entry.path();

    let is_ignored_dir =
        path.is_dir() && path.file_name().unwrap().to_str().unwrap() == "node_modules";

    let is_ignored_file = path.is_file() && !should_process_file(path);

    is_dot || is_ignored_dir || is_ignored_file
}
