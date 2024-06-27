use std::str;

pub fn get_compile_command(main_file_name: &str) -> Option<tokio::process::Command> {
    let mut command = tokio::process::Command::new("g++");
    command
        .arg(format!("{}.cpp", main_file_name))
        .arg("-o")
        .arg(main_file_name);

    Some(command)
}

pub fn get_execute_command(main_file_name: &str) -> std::process::Command {
    std::process::Command::new(format!("./{}", main_file_name))
}
