use llm_tools_config::{generate_gemini_tools_config, generate_openai_tools_config, FunctionDefinition, FunctionParameter, ToolConfig};

fn main() {
    let config = ToolConfig {
        functions: vec![
            FunctionDefinition {
                name: "get_current_weather".to_string(),
                description: "获取当前天气".to_string(),
                parameters: vec![
                    FunctionParameter {
                        name: "location".to_string(),
                        description: "城市名".to_string(),
                        r#type: "string".to_string(),
                        enum_values: None,
                        required: true,
                    },
                    FunctionParameter {
                        name: "unit".to_string(),
                        description: "温度单位".to_string(),
                        r#type: "string".to_string(),
                        enum_values: Some(vec!["celsius".to_string(), "fahrenheit".to_string()]),
                        required: false,
                    },
                ],
            },
        ],
    };

    match generate_gemini_tools_config(&config) {
        Ok(gemini_json) => println!("Gemini 配置: {}", gemini_json),
        Err(e) => eprintln!("Gemini 配置生成错误: {}", e),
    }

    match generate_openai_tools_config(&config) {
        Ok(openai_json) => println!("OpenAI 配置: {}", openai_json),
        Err(e) => eprintln!("OpenAI 配置生成错误: {}", e),
    }
}