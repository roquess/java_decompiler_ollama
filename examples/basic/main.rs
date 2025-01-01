use std::env;
use std::process;
use java_decompiler_ollama::translate_java_class;

fn main() {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: basic <class_file>");
        process::exit(1);
    }

    let class_file = &args[1];

    println!("Translating class file '{}' to Java source code", class_file);

    // Call the translation function
    match translate_java_class(class_file) {
        Ok(translated_code) => {
            println!("{}", translated_code);
        }
        Err(error) => {
            eprintln!("Error: {}", error);
        }
    }
}

