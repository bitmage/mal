use std::io::{self, Write};

fn read(input: &str) -> String {
    input.to_string()
}

fn eval(input: &str) -> String {
    input.to_string()
}

fn print(input: &str) -> String {
    input.to_string()
}

fn rep(input: &str) -> String {
    let first = read(&input);
    let second = eval(&first);
    let third = print(&second);
    third
}

fn main() -> io::Result<()> {
    loop {
        print!("user> ");
        io::stdout().flush()?;
        let mut input_buffer = String::new();
        let result = io::stdin().read_line(&mut input_buffer)?;
        if result == 0 {
            break
        }
        let evaluated = rep(&input_buffer);
        println!("{}", evaluated);
    }
    Ok(())
}
