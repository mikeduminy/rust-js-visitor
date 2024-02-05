use std::env;

pub mod extract_imports;

// Instructions:
// create a `test.js`,
// run `cargo run -p js-visitor -- --ignore-dynamic-error`

fn main() {
    let args = env::args().collect::<Vec<String>>();
    args.iter().for_each(|arg| println!("{}", arg));

    let mut name = "test.js".to_string();
    let mut panic_on_dynamic_errors = true;

    for arg in args.iter() {
        let positional = !arg.starts_with('-');

        if positional {
            name = arg.to_string()
        }

        match arg.as_str() {
            "-i" | "--ignore-dynamic-error" => panic_on_dynamic_errors = false,
            _ => (),
        }
    }

    let mut package_names = extract_imports::extract_imports(&name, panic_on_dynamic_errors);

    package_names.dedup();

    // print list to stdout
    for package_name in package_names {
        println!("{}", package_name);
    }
}
