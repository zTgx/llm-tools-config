use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FunctionParameter {
    pub name: String,
    pub description: String,
    pub r#type: String,
    pub enum_values: Option<Vec<String>>,
    pub required: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FunctionDefinition {
    pub name: String,
    pub description: String,
    pub parameters: Vec<FunctionParameter>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ToolConfig {
    pub functions: Vec<FunctionDefinition>,
}

pub fn generate_gemini_tools_config(config: &ToolConfig) -> Result<String, serde_json::Error> {
    let mut gemini_config = serde_json::json!({
        "tools": {
            "functionDeclarations": []
        }
    });

    for function in &config.functions {
        let mut parameters_properties = serde_json::Map::new();
        let mut required_parameters = Vec::new();

        for parameter in &function.parameters {
            let mut parameter_definition = serde_json::json!({
                "type": parameter.r#type,
                "description": parameter.description,
            });

            if let Some(ref enum_values) = parameter.enum_values {
                parameter_definition["enum"] = serde_json::to_value(enum_values)?;
            }

            parameters_properties.insert(parameter.name.clone(), parameter_definition);

            if parameter.required {
                required_parameters.push(parameter.name.clone());
            }
        }

        let function_declaration = serde_json::json!({
            "name": function.name,
            "description": function.description,
            "parameters": {
                "type": "object",
                "properties": parameters_properties,
                "required": required_parameters,
            },
        });

        gemini_config["tools"]["functionDeclarations"]
            .as_array_mut()
            .unwrap()
            .push(function_declaration);
    }

    serde_json::to_string_pretty(&gemini_config)
}

pub fn generate_openai_tools_config(config: &ToolConfig) -> Result<String, serde_json::Error> {
    let mut openai_config = serde_json::json!({
        "tools": []
    });

    for function in &config.functions {
        let mut parameters_properties = serde_json::Map::new();
        let mut required_parameters = Vec::new();

        for parameter in &function.parameters {
            let mut parameter_definition = serde_json::json!({
                "type": parameter.r#type,
                "description": parameter.description,
            });

            if let Some(ref enum_values) = parameter.enum_values {
                parameter_definition["enum"] = serde_json::to_value(enum_values)?;
            }

            parameters_properties.insert(parameter.name.clone(), parameter_definition);

            if parameter.required {
                required_parameters.push(parameter.name.clone());
            }
        }

        let function_tool = serde_json::json!({
            "type": "function",
            "function": {
                "name": function.name,
                "description": function.description,
                "parameters": {
                    "type": "object",
                    "properties": parameters_properties,
                    "required": required_parameters,
                },
            },
        });

        openai_config["tools"].as_array_mut().unwrap().push(function_tool);
    }

    serde_json::to_string_pretty(&openai_config)
}