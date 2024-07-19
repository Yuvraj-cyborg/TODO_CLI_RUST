use std::process::Command;

#[test]
fn test_cli_add_task() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("add")
        .arg("test_list")
        .arg("test_task")
        .output()
        .expect("Failed to execute command");

    assert!(String::from_utf8_lossy(&output.stdout).contains("Added task 'test_task' to list 'test_list'"));
}

#[test]
fn test_cli_show_tasks() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("show")
        .output()
        .expect("Failed to execute command");

    assert!(String::from_utf8_lossy(&output.stdout).contains("test_task"));
}
