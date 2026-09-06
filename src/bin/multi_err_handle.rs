use std::{collections::HashMap};

struct Card{
    number:u32,
    expiration:Expiration,
    cvv:u32
}

struct Expiration{
    month:u32,
    year:u32
}
fn main(){
    let credit_cards:HashMap<&str,&str> = HashMap::from([
        ("amy","1234567 04 25 123"),
        ("bob","1234567 06 25 123"),
        ("cathy","1234567 12 25 123"),
    ]);

    println!("enter ur name:");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim();

    let result = get_credit_card_info(&credit_cards,name);
}

fn get_credit_card_info<'a>(credit_cards:&'a HashMap<&str,&str>,name:&str)->Card{
    
    let card_string = credit_cards.get(name).unwrap(); 

    let card = parse_card(card_string);

    card
}

fn parse_card(card_string:&str)->Card{
    let mut numbers = parse_card_numbers(card_string);

    let cvv = numbers.pop().unwrap();
    let expiration_year = numbers.pop().unwrap();
    let expiration_month = numbers.pop().unwrap();
    let expiration = Expiration { month: expiration_month, year: expiration_year };
    let number = numbers.pop().unwrap();
    Card { number, expiration, cvv }
}

fn parse_card_numbers(card_string:&str)->Vec<u32>{

    let numbers = card_string.split(" ").into_iter().map(|s| s.parse::<u32>()).
    collect::<Result<Vec<u32>, _>>().unwrap();
    numbers
}