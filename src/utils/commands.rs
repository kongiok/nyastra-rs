pub fn parse_command(input: &str, prefix: &str) -> Option<(String, Vec<String>)> {
    if !input.starts_with(prefix) {
        return None;
    }
    let without_prefix = &input[1..]; // Remove prefix
    let mut parts = without_prefix.split_whitespace();

    let command = parts.next()?; // remove the command
    let args = parts.map(|s| s.to_string()).collect();

    Some((command.to_string(), vec![args]))
}
