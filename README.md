# llm-tools-config

`llm-tools-config` is a Rust library designed to simplify the generation of tools configurations for Large Language Models (LLMs), specifically targeting OpenAI, Gemini and more. It automates the creation of JSON schemas required for function calling, making it easier to integrate external tools and APIs with your LLM applications.

## Features

* **Simplified Configuration:** Generates JSON configurations for LLM function calling, reducing manual effort and potential errors.
* **Cross-Platform Compatibility:** Written in Rust, ensuring compatibility across various operating systems.
* **OpenAI and Gemini Support:** Specifically designed to generate configurations for both OpenAI and Gemini models.
* **Easy Integration:** Provides a straightforward API for defining function parameters and generating configurations.

## Getting Started

### Prerequisites

* Rust installed (version 1.76 or higher)
* Cargo package manager

### Installation

Add `llm-tools-config` to your `Cargo.toml` file:

```toml
[dependencies]
llm-tools-config = "0.1.0"
```

### Usage

Here's a basic example of how to use `llm-tools-config` to generate a tools configuration:

```rust
use llm_tools_config::{FunctionDefinition, FunctionParameter, ToolConfig, generate_gemini_tools_config, generate_openai_tools_config};

fn main() {
    let config = ToolConfig {
        functions: vec![
            FunctionDefinition {
                name: "get_current_weather".to_string(),
                description: "Get the current weather".to_string(),
                parameters: vec![
                    FunctionParameter {
                        name: "location".to_string(),
                        description: "City name".to_string(),
                        r#type: "string".to_string(),
                        enum_values: None,
                        required: true,
                    },
                    FunctionParameter {
                        name: "unit".to_string(),
                        description: "Temperature unit".to_string(),
                        r#type: "string".to_string(),
                        enum_values: Some(vec!["celsius".to_string(), "fahrenheit".to_string()]),
                        required: false,
                    },
                ],
            },
        ],
    };

    match generate_gemini_tools_config(&config) {
        Ok(gemini_json) => println!("Gemini Config: {}", gemini_json),
        Err(e) => eprintln!("Gemini Config Generation Error: {}", e),
    }

    match generate_openai_tools_config(&config) {
        Ok(openai_json) => println!("OpenAI Config: {}", openai_json),
        Err(e) => eprintln!("OpenAI Config Generation Error: {}", e),
    }
}
```

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue for bugs or feature requests.

## License

This project is licensed under the MIT License.
