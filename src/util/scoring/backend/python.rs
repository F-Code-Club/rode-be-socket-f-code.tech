use std::str;

pub fn get_compile_command() -> Option<tokio::process::Command> {
    None
}

pub fn get_execute_command(main_file_name: &str) -> std::process::Command {
    let mut command = std::process::Command::new("python");
    command.arg(format!("{}.py", main_file_name));

    command
}
