use std::env;
use std::process::Command;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, CONTENT_TYPE};
use serde_json::{json, Value};
use thiserror::Error;

/// Errors that can occur during the translation process.
#[derive(Debug, Error)]
pub enum TranslationError {
    /// Error reading the file.
    #[error("Error reading the file: {0}")]
    FileReadError(#[from] std::io::Error),
    /// Error sending request to Ollama or retrieving response.
    #[error("Error sending request to Ollama or retrieving response: {0}")]
    RequestError(#[from] reqwest::Error),
    /// Ollama error with HTTP status code.
    #[error("Ollama error: HTTP status {0}")]
    OllamaError(u16),
    /// Error parsing JSON.
    #[error("Error parsing JSON: {0}")]
    JsonError(#[from] serde_json::Error),
    /// Error running javap.
    #[error("Error running javap: {0}")]
    JavapError(String),
}

/// Configuration struct for the translation process.
struct Config {
    model: String,
    url: String,
}

impl Config {
    /// Creates a new Config from environment variables.
    fn from_env() -> Self {
        Self {
            model: env::var("OLLAMA_MODEL").unwrap_or_else(|_| "qwen2.5-coder".to_string()),
            url: env::var("OLLAMA_URL").unwrap_or_else(|_| "http://localhost:11434/api/generate".to_string()),
        }
    }
}

/// Creates the headers required for the HTTP request.
fn create_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers
}

/// Parses the response from the Ollama API and extracts the translated code.
fn parse_response(response_text: &str) -> Result<String, TranslationError> {
    let mut result = String::new();
    let mut skip_first_line = true;
    for line in response_text.lines() {
        if let Ok(json_value) = serde_json::from_str::<Value>(line) {
            if let Some(response_content) = json_value.get("response") {
                if let Some(content) = response_content.as_str() {
                    if content.contains("<|im_start|>") || content.contains("```") {
                        continue;
                    }
                    if skip_first_line && !content.trim().is_empty() {
                        skip_first_line = false;
                        continue;
                    }
                    result.push_str(content);
                }
            }
        }
    }

    let clean_result = result
        .lines()
        .skip_while(|line| line.trim().is_empty() || line.contains("<|im_start|>") || line.contains("```"))
        .take_while(|line| !line.contains("```"))
        .collect::<Vec<&str>>()
        .join("\n");

    Ok(clean_result.trim().to_string())
}

/// Disassembles a Java class file using javap and translates the bytecode to Java source code using the Ollama API.
///
/// # Arguments
///
/// * `class_file` - A string slice that holds the path to the Java class file to be disassembled.
///
/// # Returns
///
/// * `Ok(String)` - The translated Java source code as a string.
/// * `Err(TranslationError)` - An error that occurred during the translation process.
pub fn translate_java_class(class_file: &str) -> Result<String, TranslationError> {
    // Appelle javap pour désassembler la classe Java
    let output = Command::new("javap")
        .arg("-c")
        .arg(class_file)
        .output()
        .map_err(|e| TranslationError::JavapError(e.to_string()))?;

    if !output.status.success() {
        return Err(TranslationError::JavapError(String::from_utf8_lossy(&output.stderr).to_string()));
    }

    let javap_output = String::from_utf8_lossy(&output.stdout).to_string();

    // Envoie le bytecode désassemblé à Ollama
    let config = Config::from_env();
    let client = Client::new();
    let body = json!({
        "model": config.model,
        "prompt": format!(
            "Translate the following bytecode to Java source code. Output only the code:\n{}",
            javap_output
        ),
        "stream": true
    });

    let headers = create_headers();
    let response = client
        .post(&config.url)
        .headers(headers)
        .json(&body)
        .send()?;

    if !response.status().is_success() {
        return Err(TranslationError::OllamaError(response.status().as_u16()));
    }

    let response_text = response.text()?;
    let translated_code = parse_response(&response_text)?;

    Ok(translated_code)
}

