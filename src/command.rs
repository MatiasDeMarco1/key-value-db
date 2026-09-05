pub enum Command {
    Set(String, String),
    Get(String),
    Delete(String),
    Exit,
    Unknown,
}
pub fn parse(input: &str) -> Command{
    let parts: Vec<&str> = input.split_whitespace().collect();
    match parts.as_slice() {
        [cmd, key, value] if cmd.eq_ignore_ascii_case("SET") => Command::Set(key.to_string(), value.to_string()),
        [cmd, key] if cmd.eq_ignore_ascii_case("GET") => Command::Get(key.to_string()),
        [cmd, key] if cmd.eq_ignore_ascii_case("DELETE") => Command::Delete(key.to_string()),
        [cmd] if cmd.eq_ignore_ascii_case("EXIT") => Command::Exit,
        _ => Command::Unknown
    }
}