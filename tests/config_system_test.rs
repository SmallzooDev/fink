use fink::utils::config::Config;
use tempfile::TempDir;
use std::fs;

#[test]
fn should_have_default_editor() {
    let config = Config::default();
    assert_eq!(config.editor(), "vim");
}

#[test]
fn should_have_default_storage_path() {
    let config = Config::default();
    let storage_path = config.storage_path();
    assert!(storage_path.ends_with(".fink"));
}

#[test]
fn should_have_default_config_path() {
    let config_path = Config::default_config_path();
    assert!(config_path.ends_with(".config/fink/config.toml"));
}

#[test]
fn should_create_default_config_if_not_exists() {
    let temp_dir = TempDir::new().unwrap();
    let config_dir = temp_dir.path().join(".config/jkms");
    let config_file = config_dir.join("config.toml");
    
    // Ensure config doesn't exist
    assert!(!config_file.exists());
    
    // Load or create config
    Config::ensure_config_exists(&config_file).unwrap();
    
    // Config file should now exist
    assert!(config_file.exists());
    
    // Verify content
    let content = fs::read_to_string(&config_file).unwrap();
    assert!(content.contains("editor = \"vim\""));
    assert!(content.contains("storage_path ="));
}

#[test]
fn should_load_config_from_file() {
    let temp_dir = TempDir::new().unwrap();
    let config_file = temp_dir.path().join("config.toml");
    
    let config_content = r#"
editor = "nvim"
storage_path = "/custom/path/prompts"
"#;
    
    fs::write(&config_file, config_content).unwrap();
    
    let config = Config::load_from_file(&config_file).unwrap();
    
    assert_eq!(config.editor(), "nvim");
    assert_eq!(config.storage_path().to_str().unwrap(), "/custom/path/prompts");
}

#[test]
fn should_load_or_create_default_config() {
    let temp_dir = TempDir::new().unwrap();
    let config_dir = temp_dir.path().join(".config/jkms");
    let config_file = config_dir.join("config.toml");
    
    // First time - should create default
    let config = Config::load_or_create(&config_file).unwrap();
    assert_eq!(config.editor(), "vim");
    assert!(config.storage_path().ends_with(".fink"));
    
    // File should be created
    assert!(config_file.exists());
    
    // Modify the file
    let custom_content = r#"
editor = "helix"
storage_path = "/my/custom/path"
"#;
    fs::write(&config_file, custom_content).unwrap();
    
    // Second time - should load from file
    let config2 = Config::load_or_create(&config_file).unwrap();
    assert_eq!(config2.editor(), "helix");
    assert_eq!(config2.storage_path().to_str().unwrap(), "/my/custom/path");
}