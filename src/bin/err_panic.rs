use anyhow::{Context, Result};


// enum MyError {
//     Io(std::io::Error),
//     Num(std::num::ParseIntError),
// }


fn get_int_from_file() -> Result<i32> {
    let path = "number.txt";
    let num_str = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read string from {}", path))?;
    
    return num_str
        .trim()
        .parse::<i32>()
        .map(|t| t * 2)
        .context("failed to parse string");
}


fn main() {
    match get_int_from_file() {
        Ok(x) => println!("{:#?}", x),
        Err(e) => println!("{:#?}", e),
    }
}