use std::str;

pub fn get_compile_command(main_file_name: &str) -> Option<tokio::process::Command> {
    let mut command = tokio::process::Command::new("javac");
    command.arg(format!("{}.java", main_file_name));

    Some(command)
}

pub fn get_execute_command(main_file_name: &str) -> std::process::Command {
    let mut command = std::process::Command::new("java");
    command.arg(main_file_name);

    command
}
