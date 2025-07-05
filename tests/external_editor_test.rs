use jkms::external::editor::EditorLauncher;
use std::env;
use tempfile::NamedTempFile;

#[test]
fn editor_launcher_should_use_editor_env_var() {
    // Arrange
    unsafe {
        env::set_var("EDITOR", "test-editor");
        env::remove_var("VISUAL");
    }
    let launcher = EditorLauncher::new();
    
    // Act
    let editor = launcher.get_editor();
    
    // Assert
    assert_eq!(editor, "test-editor");
    
    // Cleanup
    unsafe {
        env::remove_var("EDITOR");
    }
}

#[test]
fn editor_launcher_should_use_visual_env_var_when_editor_not_set() {
    // Arrange - save current values
    let saved_editor = env::var("EDITOR").ok();
    let saved_visual = env::var("VISUAL").ok();
    
    unsafe {
        env::remove_var("EDITOR");
        env::set_var("VISUAL", "test-visual");
    }
    let launcher = EditorLauncher::new();
    
    // Act
    let editor = launcher.get_editor();
    
    // Assert
    assert_eq!(editor, "test-visual");
    
    // Cleanup - restore original values
    unsafe {
        if let Some(val) = saved_editor {
            env::set_var("EDITOR", val);
        } else {
            env::remove_var("EDITOR");
        }
        if let Some(val) = saved_visual {
            env::set_var("VISUAL", val);
        } else {
            env::remove_var("VISUAL");
        }
    }
}

#[test]
fn editor_launcher_should_default_to_vim() {
    // Arrange - save current values
    let saved_editor = env::var("EDITOR").ok();
    let saved_visual = env::var("VISUAL").ok();
    
    unsafe {
        env::remove_var("EDITOR");
        env::remove_var("VISUAL");
    }
    let launcher = EditorLauncher::new();
    
    // Act
    let editor = launcher.get_editor();
    
    // Assert
    assert_eq!(editor, "vim");
    
    // Cleanup - restore original values
    unsafe {
        if let Some(val) = saved_editor {
            env::set_var("EDITOR", val);
        } else {
            env::remove_var("EDITOR");
        }
        if let Some(val) = saved_visual {
            env::set_var("VISUAL", val);
        } else {
            env::remove_var("VISUAL");
        }
    }
}

#[test]
fn editor_launcher_should_create_command_with_file_path() {
    // Arrange
    let launcher = EditorLauncher::new();
    let temp_file = NamedTempFile::new().unwrap();
    let file_path = temp_file.path();
    
    // Act
    let command = launcher.create_command(file_path);
    
    // Assert
    // We can't easily test Command execution in unit tests,
    // but we can verify the command is constructed
    assert!(command.get_program().to_str().is_some());
    assert!(command.get_args().any(|arg| arg == file_path.as_os_str()));
}