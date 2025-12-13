use clap::{Parser, Subcommand};
use anyhow::{Result, anyhow};
use serde_json::{Value, from_str as from_json_str};
use serde_yaml::{from_str as from_yaml_str};
use toml::{from_str as from_toml_str};
use jsonschema::{JSONSchema, ValidationError};
use std::fs;
use std::path::{Path, PathBuf};
use log::{info, debug, error};
use env_logger;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Validates a configuration file against a JSON schema
    Validate {
        /// Path to the configuration file (JSON, YAML, or TOML)
        #[clap(short, long, value_parser)]
        config_file: PathBuf,
        /// Path to the JSON schema file
        #[clap(short, long, value_parser)]
        schema_file: PathBuf,
    },
    /// Cleans a configuration file based on a JSON schema (removes unknown properties)
    Clean {
        /// Path to the configuration file (JSON, YAML, or TOML)
        #[clap(short, long, value_parser)]
        config_file: PathBuf,
        /// Path to the JSON schema file
        #[clap(short, long, value_parser)]
        schema_file: PathBuf,
        /// Output path for the cleaned configuration file (defaults to overwrite original)
        #[clap(short, long, value_parser)]
        output_file: Option<PathBuf>,
    },
    /// Generates a default configuration file based on a JSON schema
    Generate {
        /// Path to the JSON schema file
        #[clap(short, long, value_parser)]
        schema_file: PathBuf,
        /// Output path for the generated configuration file
        #[clap(short, long, value_parser)]
        output_file: PathBuf,
        /// Format of the output file (json, yaml, toml). Defaults to json.
        #[clap(short, long, value_parser, default_value = "json")]
        format: String,
    },
}

fn load_config(path: &Path) -> Result<Value> {
    let content = fs::read_to_string(path)?;
    match path.extension().and_then(|s| s.to_str()) {
        Some("json") => Ok(from_json_str(&content)?),
        Some("yaml") | Some("yml") => Ok(from_yaml_str(&content)?),
        Some("toml") => Ok(from_toml_str(&content)?),
        _ => Err(anyhow!("Unsupported config file format. Use .json, .yaml, .yml, or .toml")),
    }
}

fn load_schema(path: &Path) -> Result<Value> {
    let content = fs::read_to_string(path)?;
    Ok(from_json_str(&content)?)
}

fn validate_config(config: &Value, schema: &Value) -> Result<()> {
    let compiled_schema = JSONSchema::compile(schema).map_err(|e| anyhow!("Invalid schema: {}", e))?;
    let validation_result = compiled_schema.validate(config);

    if let Err(errors) = validation_result {
        for error in errors {
            error!("Validation error: {}", error);
            error!("  Instance path: {}", error.instance_path);
            error!("  Schema path: {}", error.schema_path);
            error!("  Kind: {:?}", error.kind);
        }
        Err(anyhow!("Configuration validation failed."))
    } else {
        info!("Configuration validated successfully.");
        Ok(())
    }
}

fn clean_config(config: &mut Value, schema: &Value) -> Result<()> {
    let schema_map = schema.as_object().ok_or_else(|| anyhow!("Schema is not a JSON object."))?;
    let properties = schema_map.get("properties").and_then(|p| p.as_object());

    if let Some(properties) = properties {
        if let Some(config_object) = config.as_object_mut() {
            let mut keys_to_remove = Vec::new();
            for (key, _value) in config_object.iter() {
                if !properties.contains_key(key) {
                    debug!("Removing unknown property: {}", key);
                    keys_to_remove.push(key.clone());
                }
            }
            for key in keys_to_remove {
                config_object.remove(&key);
            }
        }
    }
    info!("Configuration cleaned successfully.");
    Ok(())
}

fn generate_config_from_schema(schema: &Value) -> Result<Value> {
    let mut generated_config = Value::Object(serde_json::Map::new());
    let schema_map = schema.as_object().ok_or_else(|| anyhow!("Schema is not a JSON object."))?;

    if let Some(properties) = schema_map.get("properties").and_then(|p| p.as_object()) {
        for (key, schema_property) in properties {
            if let Some(default_value) = schema_property.get("default") {
                generated_config[key] = default_value.clone();
                debug!("Generated property '{}' with default value: {:?}", key, default_value);
            } else if let Some(schema_type) = schema_property.get("type").and_then(|t| t.as_str()) {
                // Basic type inference for generation if no default is provided
                generated_config[key] = match schema_type {
                    "string" => Value::String("".to_string()),
                    "integer" | "number" => Value::Number(0.into()),
                    "boolean" => Value::Bool(false),
                    "array" => Value::Array(vec![]),
                    "object" => Value::Object(serde_json::Map::new()),
                    _ => Value::Null, // Fallback for unknown types
                };
                debug!("Generated property '{}' with inferred type: {}", key, schema_type);
            }
        }
    }
    info!("Default configuration generated.");
    Ok(generated_config)
}

fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    match &cli.command {
        Commands::Validate { config_file, schema_file } => {
            info!("Starting validation for config: {:?}, schema: {:?}", config_file, schema_file);
            let config = load_config(config_file)?;
            let schema = load_schema(schema_file)?;
            validate_config(&config, &schema)
        },
        Commands::Clean { config_file, schema_file, output_file } => {
            info!("Starting cleaning for config: {:?}, schema: {:?}", config_file, schema_file);
            let mut config = load_config(config_file)?;
            let schema = load_schema(schema_file)?;
            clean_config(&mut config, &schema)?;

            let output_path = output_file.as_ref().unwrap_or(config_file);
            let output_content = match output_path.extension().and_then(|s| s.to_str()) {
                Some("json") => serde_json::to_string_pretty(&config)?,
                Some("yaml") | Some("yml") => serde_yaml::to_string(&config)?,
                Some("toml") => toml::to_string_pretty(&config)?,
                _ => return Err(anyhow!("Unsupported output format for cleaning. Use .json, .yaml, .yml, or .toml")),
            };
            fs::write(output_path, output_content)?;
            info!("Cleaned configuration saved to: {:?}", output_path);
            Ok(())
        },
        Commands::Generate { schema_file, output_file, format } => {
            info!("Starting generation for schema: {:?}, output: {:?}, format: {}", schema_file, output_file, format);
            let schema = load_schema(schema_file)?;
            let generated_config = generate_config_from_schema(&schema)?;

            let output_content = match format.as_str() {
                "json" => serde_json::to_string_pretty(&generated_config)?,
                "yaml" => serde_yaml::to_string(&generated_config)?,
                "toml" => toml::to_string_pretty(&generated_config)?,
                _ => return Err(anyhow!("Unsupported output format. Use json, yaml, or toml")),
            };
            fs::write(output_file, output_content)?;
            info!("Generated configuration saved to: {:?}", output_file);
            Ok(())
        },
    }
}
