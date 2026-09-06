struct Credentials<T>
where
    T: Fn(&str, &str) -> bool,
{
    username: String,
    password: String,
    validator: T,
}

impl<T> Credentials<T>
where
    T: Fn(&str, &str) -> bool,
{
    fn is_valid(&self) -> bool {
        (self.validator)(&self.username, &self.password)
    }
}

fn main() {
    let weak_password = "password123!".to_owned();
    let validator = |username: &str, password: &str| {
        !username.is_empty() && !password.is_empty() && password != weak_password
    };

    //Fn - Immutable closure, FnMut - Mutable closure, FnOnce - Consuming closure

    let credentials = Credentials {
        username: String::from("user123"),
        password: String::from("pass123"),
        validator,
    };
    println!("Weak password: {weak_password}");
    println!(
        "Are the credentials valid? {}",
        validate_credentials(&credentials.username, &credentials.password)
    );
    println!(
        "Are the credentials valid? {}",
        validator(&credentials.username, &credentials.password)
    );
    println!("{}", credentials.is_valid());

    let default_creds = get_default_creds(get_password_validator(10, true));
    println!(
        "Default credentials: username: {}, password: {}, is valid: {}",
        default_creds.username,
        default_creds.password,
        default_creds.is_valid()
    );
}

fn validate_credentials(username: &str, password: &str) -> bool {
    !username.is_empty() && !password.is_empty()
}

fn get_default_creds<T>(f: T) -> Credentials<T>
where
    T: Fn(&str, &str) -> bool,
{
    Credentials {
        username: "guest".to_owned(),
        password: "password123!".to_owned(),
        validator: f,
    }
}

fn get_password_validator(min_len: usize, special_char: bool) -> Box<dyn Fn(&str, &str) -> bool> {
    if special_char {
        Box::new(move |_: &str, password: &str| {
            password.len() >= min_len && password.chars().any(|c| !c.is_alphanumeric())
        })
    } else {
        Box::new(move |_: &str, password: &str| password.len() >= min_len)
    }
}
