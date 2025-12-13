use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "guardian",
    version,
    about = "Strict, fast configuration validation for CI/CD"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Validate a configuration file against a JSON Schema
    Validate {
        /// Path to config file (yaml/json/toml)
        config: String,

        /// Path to JSON Schema file
        schema: String,

        /// Inject default values from schema
        #[arg(long)]
        inject_defaults: bool,

        /// Enable basic security heuristics
        #[arg(long)]
        security: bool,

        /// Fail validation on security warnings
        #[arg(long)]
        strict_security: bool,

        /// Recursively validate directories
        #[arg(long)]
        recursive: bool,

        /// Write cleaned config back to disk
        #[arg(long)]
        write: bool,
    },

    /// Generate a draft JSON Schema from a config file
    GenerateSchema {
        /// Path to config file
        config: String,
    },
}
