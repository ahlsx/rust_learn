use std::ops::{Deref, DerefMut};

struct MySmartPointer<T> {
    data: T,
}

impl<T> MySmartPointer<T> {
    fn new(data: T) -> Self {
        MySmartPointer { data }
    }
}

impl<T> Deref for MySmartPointer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for MySmartPointer<T> {
    //type Target = T;
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

fn main() {
    let s = MySmartPointer::new(String::from("Hello, world!"));
    //&MySmartPointer -> &String -> &str
    print(&s);
}

fn print(s: &str) {
    println!("{}", s);
}
