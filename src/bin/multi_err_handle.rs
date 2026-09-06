use std::{collections::HashMap,num::ParseIntError};

#[derive(Debug)]
struct Card{
    number:u32,
    expiration:Expiration,
    cvv:u32
}

#[derive(Debug)]
struct Expiration{
    month:u32,
    year:u32
}
fn main(){
    let credit_cards:HashMap<&str,&str> = HashMap::from([
        ("amy","1234567 04 25 123"),
        ("bob","1234567 06 25 123"),
        ("cathy","1234567 25 123"),
    ]);

    println!("enter ur name:");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim();

    match get_credit_card_info(&credit_cards,name){
        Ok(card) => {
            println!("\nCreditcard info: {card:?}");
        },
        Err(e) => {
            println!("{}", e);
        }
    }
}

fn get_credit_card_info<'a>(credit_cards:&'a HashMap<&str,&str>,name:&str)->Result<Card, String>{
    
    let card_string = credit_cards.get(name).ok_or(format!("No credit card info found for user: {}", name))?; 

    let card = parse_card(card_string)?;

    Ok(card)
}

fn parse_card(card_string:&str)->Result<Card, String>{
    let mut numbers = parse_card_numbers(card_string).map_err(|e| e.to_string())?;

    if numbers.len() != 4 {
        return Err(format!("Incorrect number of card details. Expected 4, got {},{numbers:?}", numbers.len()));
    }

    let cvv = numbers.pop().unwrap();
    let expiration_year = numbers.pop().unwrap();
    let expiration_month = numbers.pop().unwrap();
    let expiration = Expiration { month: expiration_month, year: expiration_year };
    let number = numbers.pop().unwrap();

    
    Ok(Card { number, expiration, cvv })
}

fn parse_card_numbers(card_string:&str)->Result<Vec<u32>, ParseIntError>{

    let numbers = card_string.split(" ").into_iter().map(|s| s.parse::<u32>()).
    collect::<Result<Vec<u32>, _>>()?;
    Ok(numbers)
}