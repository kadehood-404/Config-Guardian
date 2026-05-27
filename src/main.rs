use std::fs;
use std::collections::HashSet;
use sha2::{Sha256, Digest};

mod cli;

fn main() {
    println!("🛡️ CONFIG-GUARDIAN // INITIALIZING CRYPTOGRAPHIC DISCREPANCY AUDIT...");

    // 1. Parse arguments via CLI config layer
    let config = match cli::CliConfig::parse() {
        Ok(cfg) => cfg,
        Err(err) => {
            eprintln!("❌ CLI ERROR: {}", err);
            std::process::exit(1);
        }
    };

    // 2. Validate file presence
    if !config.file_path.exists() {
        eprintln!("❌ CONFIG ERROR: Target layout payload missing at {:?}", config.file_path);
        std::process::exit(1);
    }

    // 3. Read stream state into memory
    let contents = match fs::read_to_string(&config.file_path) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("❌ IO ERROR: Failure reading payload: {}", e);
            std::process::exit(1);
        }
    };

    // 4. Extract and check the signature header prefix
    let mut lines = contents.lines();
    let first_line = match lines.next() {
        Some(line) => line,
        None => {
            eprintln!("❌ SECURITY FAILURE: Payload is completely empty.");
            std::process::exit(1);
        }
    };

    if !first_line.starts_with("# @guardian-signature:") {
        eprintln!("❌ SECURITY FAILURE: Missing mandatory cryptographic policy header (# @guardian-signature:)");
        std::process::exit(1);
    }

    let expected_signature = first_line
        .trim_start_matches("# @guardian-signature:")
        .trim();

    if expected_signature.len() != 64 {
        eprintln!("❌ SECURITY FAILURE: Invalid SHA-256 signature length in header.");
        std::process::exit(1);
    }

    // 5. Isolate the body and compute the real-time checksum
    let body_lines: Vec<&str> = lines.collect();
    let body = body_lines.join("\n");
    let mut hasher = Sha256::new();
    hasher.update(body.as_bytes());
    let computed_signature = format!("{:x}", hasher.finalize());

    println!("Scanning asset layout: {:?}", config.file_path);
    println!("🔍 Expected: {}", expected_signature);
    println!("🛡️ Computed: {}", computed_signature);

    // 6. Enforce strict invariant zero-drift boundary
    if expected_signature != computed_signature {
        eprintln!("❌ CRITICAL SECURITY VIOLATION: Payload cryptographic checksum mismatch. Configuration tampering detected.");
        std::process::exit(1);
    }

    println!("✅ CRYPTOGRAPHIC INTEGRITY VERIFIED: Zero-drift baseline maintained.");
    
    // 7. Execute Strict Lexical Tokenizer Pass (Duplicate Key Guard)
    println!("🔬 STARTING AST KEY INVARIANT CHECK...");
    let mut seen_keys = HashSet::new();

    for (index, line) in body_lines.iter().enumerate() {
        let trimmed = line.trim();
        
        // Skip completely empty lines or internal file comments
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with("//") {
            continue;
        }

        // Locate mapping separator
        if let Some(pos) = trimmed.find(':') {
            let raw_key = &trimmed[..pos];
            
            // Normalize token by removing common JSON/YAML punctuation bounds
            let clean_key = raw_key
                .trim()
                .trim_matches(|c| c == '"' || c == '\'' || c == '{' || c == '}');

            if !clean_key.is_empty() {
                // If insert returns false, the set already contained the token
                if !seen_keys.insert(clean_key.to_string()) {
                    eprintln!(
                        "❌ CRITICAL CONFIGURATION FAILURE: Duplicate mapping key detected: \"{}\" on structural body line {}",
                        clean_key,
                        index + 2 // Account for 0-index offset + signature header line
                    );
                    std::process::exit(1);
                }
            }
        }
    }

    println!("✅ AST INTEGRITY VERIFIED: Zero duplicate mapping invariants discovered.");
}
