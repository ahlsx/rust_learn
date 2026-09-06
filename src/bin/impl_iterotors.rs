struct Person {
    first_name: String,
    last_name: String,
    occupation: String,
}

struct PersonIter {
    values: Vec<String>,
}

impl Iterator for PersonIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.values.is_empty() {
            None
        } else {
            Some(self.values.remove(0))
        }
    }
}

impl IntoIterator for Person {
    type Item = String;
    type IntoIter = PersonIter;

    fn into_iter(self) -> Self::IntoIter {
        PersonIter {
            values: vec![self.first_name, self.last_name, self.occupation],
        }
    }
}

fn main() {
    let person = Person {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        occupation: "Software Engineer".to_string(),
    };

    for value in person {
        println!("{}", value);
    }
}
