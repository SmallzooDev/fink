use std::fs;
use std::path::{Path, PathBuf};
use anyhow::Result;

const VERSION_FILE: &str = ".last_version";
const CHANGELOG_FILE: &str = "CHANGELOG.md";

pub struct VersionTracker {
    config_dir: PathBuf,
    current_version: String,
}

impl VersionTracker {
    pub fn new(config_dir: PathBuf) -> Self {
        Self {
            config_dir,
            current_version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
    
    /// Check if this is a new version since last run
    pub fn is_updated(&self) -> Result<bool> {
        let version_file = self.config_dir.join(VERSION_FILE);
        
        if !version_file.exists() {
            // First run, not an update
            return Ok(false);
        }
        
        let stored_version = fs::read_to_string(&version_file)?;
        Ok(self.version_newer_than(&stored_version))
    }
    
    /// Get the previously stored version
    pub fn get_previous_version(&self) -> Result<Option<String>> {
        let version_file = self.config_dir.join(VERSION_FILE);
        
        if !version_file.exists() {
            return Ok(None);
        }
        
        Ok(Some(fs::read_to_string(&version_file)?))
    }
    
    /// Store the current version
    pub fn store_current_version(&self) -> Result<()> {
        let version_file = self.config_dir.join(VERSION_FILE);
        fs::create_dir_all(&self.config_dir)?;
        fs::write(version_file, &self.current_version)?;
        Ok(())
    }
    
    /// Get update notes from CHANGELOG for current version
    pub fn get_update_notes(&self) -> Result<Option<String>> {
        // Try to find CHANGELOG.md in various locations
        let possible_paths = vec![
            Path::new(CHANGELOG_FILE).to_path_buf(),
            self.config_dir.parent().unwrap_or(Path::new(".")).join(CHANGELOG_FILE),
            self.config_dir.parent().and_then(|p| p.parent()).unwrap_or(Path::new(".")).join(CHANGELOG_FILE),
            Path::new("/usr/local/share/fink").join(CHANGELOG_FILE),
        ];
        
        for path in possible_paths {
            if path.exists() {
                let content = fs::read_to_string(path)?;
                return Ok(self.parse_changelog_for_version(&content));
            }
        }
        
        // If no CHANGELOG found, return generic message
        Ok(Some(format!("Updated to version {}", self.current_version)))
    }
    
    fn version_newer_than(&self, other: &str) -> bool {
        if self.current_version == other {
            return false;
        }
        
        let current_parts: Vec<u32> = self.current_version
            .split('.')
            .filter_map(|s| s.parse().ok())
            .collect();
        let other_parts: Vec<u32> = other
            .split('.')
            .filter_map(|s| s.parse().ok())
            .collect();
        
        for i in 0..3 {
            let c = current_parts.get(i).copied().unwrap_or(0);
            let o = other_parts.get(i).copied().unwrap_or(0);
            if c > o {
                return true;
            } else if c < o {
                return false;
            }
        }
        false
    }
    
    fn parse_changelog_for_version(&self, changelog: &str) -> Option<String> {
        let version_header = format!("## [{}]", self.current_version);
        let lines: Vec<&str> = changelog.lines().collect();
        
        let mut version_start = None;
        for (i, line) in lines.iter().enumerate() {
            if line.starts_with(&version_header) {
                version_start = Some(i);
                break;
            }
        }
        
        if let Some(start) = version_start {
            let mut notes = Vec::new();
            
            // Add version header
            notes.push(format!("# What's New in v{}", self.current_version));
            notes.push("".to_string());
            
            for i in (start + 1)..lines.len() {
                let line = lines[i];
                // Stop at next version header
                if line.starts_with("## [") {
                    break;
                }
                // Skip empty lines at the beginning
                if notes.len() == 2 && line.trim().is_empty() {
                    continue;
                }
                notes.push(line.to_string());
            }
            
            if notes.len() > 2 {
                return Some(notes.join("\n"));
            }
        }
        
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_version_tracking() {
        let temp = tempdir().unwrap();
        let tracker = VersionTracker::new(temp.path().to_path_buf());
        
        // First run - no update
        assert!(!tracker.is_updated().unwrap());
        assert!(tracker.get_previous_version().unwrap().is_none());
        
        // Store version
        tracker.store_current_version().unwrap();
        
        // Same version - no update
        assert!(!tracker.is_updated().unwrap());
        
        // Simulate old version
        let version_file = temp.path().join(VERSION_FILE);
        fs::write(version_file, "0.1.0").unwrap();
        
        // Should detect update
        assert!(tracker.is_updated().unwrap());
        assert_eq!(tracker.get_previous_version().unwrap(), Some("0.1.0".to_string()));
    }
}