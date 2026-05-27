use std::path::PathBuf;

pub struct CliConfig {
    pub file_path: PathBuf,
}

impl CliConfig {
    pub fn parse() -> Result<Self, String> {
        // Simple command-line argument extraction
        let args: Vec<String> = std::env::args().collect();
        if args.len() < 2 {
            return Err("Usage: config_guardian <path_to_payload.json>".to_string());
        }

        let target = PathBuf::from(&args[1]);
        Ok(CliConfig { file_path: target })
    }
}
