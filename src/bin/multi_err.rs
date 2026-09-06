use std::{fs,io, num::ParseIntError};

fn main(){
    let i = parse_file_with_custom_error("test.txt");
    match i{
        Ok(i) => println!("Parsed integer: {}",i),
        Err(e) => match e{
            ParseFileError::File => println!("Failed to read file"),
            ParseFileError::Parse(e) => println!("Failed to parse integer: {}",e),
        }
    }
    
}

enum ParseFileError{
    File,
    Parse(ParseIntError),
}

fn parse_file(file_path:&str) ->Result<i32,Box<dyn std::error::Error>>{
    let s = fs::read_to_string(file_path)?;
    let i = s.parse()?;
    Ok(i)
}

fn parse_file_with_custom_error(file_path:&str) ->Result<i32,ParseFileError>{
    let s = fs::read_to_string(file_path).map_err(|_|ParseFileError::File)?;
    let i = s.parse().map_err(|e|ParseFileError::Parse(e))?;
    Ok(i)
}
    