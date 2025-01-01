# Java Decompiler Ollama

This Rust library provides functionality to disassemble Java class files using `javap` and translate the bytecode to equivalent Java source code using the Ollama API.

## Requirements

- **Ollama**: Ensure that the Ollama API is accessible. By default, the library uses the `qwen2.5-coder` model, but you can change this by setting the `OLLAMA_MODEL` environment variable.
- **Java Development Kit (JDK)**: Ensure that `javap` is available in your environment, as it is used to disassemble the `.class` files.

## Installation

Add the following dependency to your `Cargo.toml`:

```toml
[dependencies]
java_decompiler_ollama = 0.1.0
```

## Usage

Example: Fibonacci Class
### Create a Java class file: Save the following Java code in a file named Fibonacci.java:

```java
public class Fibonacci {
    public static void main(String[] args) {
        int n = 30;
        System.out.println("Fibonacci(" + n + ") = " + fibonacci(n));
    }

    public static int fibonacci(int n) {
        if (n <= 1) {
            return n;
        }
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}
```

### Compile the Java class:

```bash
javac Fibonacci.java
```

This will generate a Fibonacci.class file.

Place the Fibonacci.class file in the test/dataset/ directory of your Rust project.


### Run the Rust example:

```bash
cargo run --example basic -- test/dataset/Fibonacci.class
```

This command will disassemble the Fibonacci.class file and print the equivalent Java source code.

## Environment Variables

You can customize the behavior of the library by setting the following environment variables:

OLLAMA_MODEL: The Ollama model to use for translation (default: qwen2.5-coder).

OLLAMA_URL: The URL of the Ollama API (default: http://localhost:11434/api/generate).

# License
This project is licensed under the MIT License. See the LICENSE file for details.

