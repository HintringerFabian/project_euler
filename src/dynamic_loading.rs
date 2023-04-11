use std::{env, fs};
use std::path::PathBuf;
use std::process::Command;
use libloading::{Library, Symbol};

type SolveFn = unsafe extern "C" fn() -> i64;
type InfoFn = unsafe extern "C" fn() -> String;

#[allow(dead_code)]
fn get_rust_files() -> Vec<PathBuf> {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let problems_dir = current_dir.join("problems");

    let rust_files = fs::read_dir(problems_dir)
        .expect("Failed to read directory")
        .filter_map(Result::ok)
        .filter(|entry| {
            let file_name = entry.file_name();
            file_name
                .to_str()
                .map_or(false, |s| s.ends_with(".rs"))
        })
        .map(|entry| entry.path())
        .collect::<Vec<PathBuf>>();

    rust_files
}

fn get_rust_files_containing(name_piece: String) -> PathBuf {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let problems_dir = current_dir.join("src/problems");

    let rust_files = fs::read_dir(problems_dir)
        .expect("Failed to read directory")
        .filter_map(Result::ok)
        .filter(|entry| {
            let file_name = entry.file_name();
            file_name
                .to_str()
                .map_or(
                    false,
                    |s| s.ends_with(".rs") && s.contains(&name_piece)
                )
        })
        .map(|entry| entry.path())
        .collect::<Vec<PathBuf>>();

    match rust_files.len() {
        0 => {
            println!("No files found");
            std::process::exit(0);
        }
        1 => rust_files[0].clone(),
        _ => {
            println!("Multiple files found");
            std::process::exit(0);
        }
    }
}

pub fn compile_files(rust_file: &PathBuf) {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let lib_dir = current_dir.join("target/debug");
    fs::create_dir_all(&lib_dir).expect("Failed to create directory");

    let lib_file =
        lib_dir
            .join(
                rust_file
                    .file_stem()
                    .expect("Failed to get file stem")
            )
            .with_extension("so");

    let command_args = &[
        rust_file.to_str().expect("Failed to convert path to string"),
        "--crate-type=dylib",
        "-o",
        lib_file.to_str().expect("Failed to convert path to string")
    ];

    let mut command = Command::new("rustc");
    command.args(command_args);
    let output = command.output();
    match output {
        Ok(output) => {
            if !output.status.success() {
                println!("Failed to compile file");
                println!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
                println!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
                std::process::exit(0);
            }
        }
        Err(_) => {
            println!("Failed to compile file");
            std::process::exit(0);
        }
    }
}

unsafe fn load_library(rust_file: PathBuf) -> Library {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let lib_dir = current_dir.join("target/debug");
    let lib_file =
        lib_dir
            .join(
                rust_file
                    .file_stem()
                    .expect("Failed to get file stem")
            )
            .with_extension("so");
    let lib = Library::new(lib_file).expect("failed to load library");
    lib
}

fn load_solve(library: &Library) -> Symbol<SolveFn> {
    unsafe {
        let solve = Library::get(library, b"solve");
        match solve {
            Ok(solve) => solve,
            Err(_) => {
                println!("No solve function found");
                std::process::exit(0);
            }
        }
    }
}

fn load_info(library: &Library) -> Symbol<InfoFn> {
    unsafe {
        let info = library.get(b"info");
        match info {
            Ok(info) => info,
            Err(_) => {
                println!("No info function found");
                std::process::exit(0);
            }
        }
    }
}

pub fn run_problem(num: String) {
    let rust_file = get_rust_files_containing(num);

    compile_files(&rust_file);

    let library = unsafe {
        let lib = load_library(rust_file);
        lib
    };

    let info = load_info(&library);
    let solve = load_solve(&library);

    unsafe {

        println!("{}", info());
        let result = solve();
        println!("Result: {}", result);
    }
}