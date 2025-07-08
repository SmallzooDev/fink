use fink::utils::config::Config;
use tempfile::tempdir;
use std::fs;
use std::path::PathBuf;

#[test]
fn test_version_update_detection() {
    let temp_dir = tempdir().unwrap();
    let config_dir = temp_dir.path().join(".config").join("fink");
    fs::create_dir_all(&config_dir).unwrap();
    
    // Test first launch - no version file exists
    let version_file = config_dir.join(".last_version");
    assert!(!version_file.exists());
    
    // Simulate app storing current version
    let current_version = env!("CARGO_PKG_VERSION");
    fs::write(&version_file, current_version).unwrap();
    
    // Read stored version
    let stored_version = fs::read_to_string(&version_file).unwrap();
    assert_eq!(stored_version, current_version);
    
    // Simulate version update
    let old_version = "0.1.0";
    fs::write(&version_file, old_version).unwrap();
    
    // Check if update detected
    let stored_version = fs::read_to_string(&version_file).unwrap();
    assert_ne!(stored_version, current_version);
    assert!(version_updated(&stored_version, current_version));
}

#[test]
fn test_version_comparison() {
    // Test various version comparisons
    assert!(version_updated("0.1.0", "0.1.1"));
    assert!(version_updated("0.1.1", "0.2.0"));
    assert!(version_updated("0.9.9", "1.0.0"));
    assert!(!version_updated("0.1.1", "0.1.1"));
    assert!(!version_updated("1.0.0", "0.9.9"));
}

#[test]
fn test_changelog_parsing() {
    let changelog_content = r#"# Changelog

## [0.1.2] - 2025-01-08

### Fixed
- Fixed Unicode input handling for Korean and other multi-byte characters
- Fixed backspace crash in search functionality

### Added
- Comprehensive Unicode support across all input fields

## [0.1.1] - 2025-01-07

### Changed
- Major refactoring
"#;
    
    // Parse changelog for specific version
    let notes = parse_changelog_for_version(changelog_content, "0.1.2");
    assert!(notes.is_some());
    assert!(notes.unwrap().contains("Fixed Unicode input handling"));
    
    // Test non-existent version
    let notes = parse_changelog_for_version(changelog_content, "0.1.3");
    assert!(notes.is_none());
}

fn version_updated(stored: &str, current: &str) -> bool {
    // Simple version comparison - in real implementation would use semver
    stored != current && {
        let stored_parts: Vec<u32> = stored.split('.').filter_map(|s| s.parse().ok()).collect();
        let current_parts: Vec<u32> = current.split('.').filter_map(|s| s.parse().ok()).collect();
        
        for i in 0..3 {
            let s = stored_parts.get(i).copied().unwrap_or(0);
            let c = current_parts.get(i).copied().unwrap_or(0);
            if c > s {
                return true;
            } else if c < s {
                return false;
            }
        }
        false
    }
}

fn parse_changelog_for_version(changelog: &str, version: &str) -> Option<String> {
    let version_header = format!("## [{}]", version);
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
        for i in (start + 1)..lines.len() {
            let line = lines[i];
            // Stop at next version header
            if line.starts_with("## [") {
                break;
            }
            notes.push(line);
        }
        
        if !notes.is_empty() {
            return Some(notes.join("\n").trim().to_string());
        }
    }
    
    None
}