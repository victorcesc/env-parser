use std::fs;
// use std::io::{self, Write};
use std::path::Path;
use crate::errors::ConfigError;

pub fn format_env_file<P: AsRef<Path>>(path: P) -> Result<String, ConfigError> {
    let content = fs::read_to_string(&path).map_err(|e| {
        ConfigError::LoadError(format!("failed to read file: {}", e))
    })?;
    let formatted = format_env_content(&content)?;
    Ok(formatted)
}

pub fn format_env_content(content: &str) -> Result<String, ConfigError> {
    let mut result = String::new();

    for line in content.lines() {
        let formatted_line = format_line(line);
        result.push_str(&formatted_line);
        result.push_str("\n");
    }
    

    // Remove trailing newline
    if result.ends_with("\n") {
        result.pop();
    }

    Ok(result)
}

fn format_line(line: &str) -> String {
    let trimmed = line.trim_end();
    if trimmed.is_empty() || trimmed.starts_with("#") {
        return trimmed.to_string();
    }

    if let Some(equal_pos) = trimmed.find('=') {
        let key = &trimmed[..equal_pos].trim_end();
        let value = &trimmed[equal_pos + 1..];


        let cleaned_value = remove_quotes(value);

        format!("{}={}", key, cleaned_value)
    } else {
        trimmed.to_string()
    }
}

fn remove_quotes(value: &str) -> String {
   let trimmed = value.trim();

    
    // Check for double quotes at start and end
    if trimmed.len() >= 2 
        && trimmed.starts_with('"') 
        && trimmed.ends_with('"') 
        && !trimmed[1..trimmed.len()-1].contains('"') {
        return trimmed[1..trimmed.len()-1].to_string();
    }
    
    // Check for single quotes at start and end
    if trimmed.len() >= 2 
        && trimmed.starts_with('\'') 
        && trimmed.ends_with('\'') 
        && !trimmed[1..trimmed.len()-1].contains('\'') {
        return trimmed[1..trimmed.len()-1].to_string();
    }
    
    // No surrounding quotes, return trimmed
    trimmed.to_string()
}

pub fn write_formatted_file<P: AsRef<Path>>(path: P, content: &str) -> Result<(), ConfigError> {
    fs::write(&path, content).map_err(|e| {
        ConfigError::LoadError(format!("failed to write file: {}", e))
    })?;
    
    Ok(())
}
