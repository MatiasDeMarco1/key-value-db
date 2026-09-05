#[derive(Debug, PartialEq)]
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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_set() {
        let result = parse("SET nombre juan");
        assert_eq!(result, Command::Set("nombre".to_string(), "juan".to_string()));
    }

    #[test]
    fn test_parse_set_case_insensitive() {
        let result = parse("set nombre juan");
        assert_eq!(result, Command::Set("nombre".to_string(), "juan".to_string()));
    }
    #[test]
    fn test_many_arguments(){
        let result = parse("GET animal nombre");
        assert_eq!(result, Command::Unknown);
    }
    #[test]
    fn void_input(){
        let result = parse("");
        assert_eq!(result,Command::Unknown);
    }
    #[test]
    fn mix_case(){
        let result = parse("eXiT");
        assert_eq!(result, Command::Exit);
    }
}