use std::io::Write;

pub mod message;
pub type UserId = u64;

pub const SERVER_ADDR: &str = "0.0.0.0:25550";





pub fn get_console_input() -> String {
    print!("Enter a message: ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}