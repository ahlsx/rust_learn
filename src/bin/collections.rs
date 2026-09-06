use std::collections::HashMap;



fn main(){
    let mut scores:HashMap<String,u32> = HashMap::new();

    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Yellow"),50);
    scores.insert(String::from("Red"),30);

    //iter() imutable reference to each key-value pair in the HashMap
    //iter_mut() mutable reference to each key-value pair in the HashMap
    //into_iter() takes ownership of the HashMap and returns an iterator that yields each key-value pair by value

    for (key,value) in &mut scores{
        *value += 1;
        println!("{}: {}",key,value);
    }
}