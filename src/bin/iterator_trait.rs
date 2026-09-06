struct MyStruct {}

/* 
trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
*/
trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}

/* 
impl Iterator for MyStruct {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(1)
    }
}
*/

impl Iterator<i32> for MyStruct {
    fn next(&mut self) -> Option<i32> {
        Some(1)
    }
}
impl Iterator<String> for MyStruct {
    fn next(&mut self) -> Option<String> {
        Some("Hello".to_string())
    }
}
/* 
trait IntoIterator {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter;
}
    */

fn main() {
    let mut iter = MyStruct {};
    let item:Option<String> = iter.next();
}
