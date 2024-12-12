pub fn parse_args() -> Result<u8, String> {
    let args: Vec<String> = std::env::args().collect();
    if let Some(arg) = args.get(1) {
        match arg.parse::<u8>() {
            Ok(num) if (1..=25).contains(&num) => Ok(num),
            Ok(_) => Err("Day must be between 1 and 25".into()),
            Err(_) => Err("Invalid number".into()),
        }
    } else {
        Err(format!("Usage: {} <day>", args[0]))
    }
}
