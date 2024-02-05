use std::env;

use walkdir::{DirEntry, WalkDir};

pub mod extract_imports;

// Instructions:
// create a `test.js`,
// run `cargo run -p js-visitor -- --ignore-dynamic-error`

fn main() {
    let args = env::args().collect::<Vec<String>>();
    args.iter().for_each(|arg| println!("{}", arg));

    let mut files = vec![];

    let mut panic_on_dynamic_errors = true;

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

    fn is_compatible_file(entry: &DirEntry) -> bool {
        entry
            .file_name()
            .to_str()
            .map(|s| s.ends_with(".js") || s.ends_with(".ts"))
            .unwrap_or(true)
    }

    fn is_ignored_entry(entry: &DirEntry) -> bool {
        entry
            .file_name()
            .to_str()
            .map(|s| s.starts_with('.') || s == "node_modules")
            .unwrap_or(false)
    }

    let mut package_names = vec![];

    println!("Files: {:?}", files);

    for file in &files {
        println!("Processing file: {}", file);
        let path = std::path::Path::new(&file);
        if path.is_file() {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            let compatible = file_name.ends_with(".js") || file_name.ends_with(".ts");
            if !compatible {
                println!("Skipping file: {}", file);
                continue;
            }
            let mut package_names_for_file =
                extract_imports::extract_imports(file, panic_on_dynamic_errors);
            package_names.append(&mut package_names_for_file);
        } else if path.is_dir() {
            println!("Processing directory: {}", file);
            let walker = WalkDir::new(file).into_iter();
            for entry in walker.filter_entry(|e| !is_ignored_entry(e)) {
                let entry = entry.unwrap();
                if !is_compatible_file(&entry) {
                    continue;
                }

                let path = entry.path();
                if path.is_file() {
                    let file_name = path.to_str().unwrap().to_string();
                    let mut package_names_for_file =
                        extract_imports::extract_imports(&file_name, panic_on_dynamic_errors);
                    package_names.append(&mut package_names_for_file);
                }
            }
        } else {
            let mut package_names_for_file =
                extract_imports::extract_imports(file, panic_on_dynamic_errors);
            package_names.append(&mut package_names_for_file);
        }
    }

    package_names.dedup();

    // print list to stdout
    for package_name in package_names {
        println!("{}", package_name);
    }
}
