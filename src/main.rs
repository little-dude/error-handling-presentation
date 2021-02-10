// mod no_error_handling;
// use no_error_handling::execute_file;

// mod anyhow_error_handling;
// use anyhow_error_handling::execute_file;

// mod manual_error_handling;
// use manual_error_handling::execute_file;

mod thiserror_error_handling;
use thiserror_error_handling::execute_file;

fn main() {
    run();
}

pub fn run() {
    for path in paths() {
        println!("executing {}", path);
        if let Err(e) = execute_file(path) {
            eprintln!("execution failed: {}", e);
        }
    }
}

// pub fn run() {
//     for path in paths() {
//         println!("executing {}", path);
//         execute_file(path);
//     }
// }

fn paths() -> Vec<&'static str> {
    vec![
        "./data/ok.json",
        "./data/invalid_file.json",
        "./data/malformed.json",
        "./data/div_by_zero.json",
    ]
}
