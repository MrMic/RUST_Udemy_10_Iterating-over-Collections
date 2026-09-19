use std::collections::HashMap;

// INFO: ---------------------------------------------------
// INFO:    Iterating Through Collections
// INFO: ---------------------------------------------------
fn main() {
    let mut vec_1 = vec![45, 30, 85, 90, 41, 39];
    // let mut vec_1_iter = vec_1.iter();
    // let mut vec_1_iter = vec_1.iter_mut();
    // let mut vec_1_iter = vec_1.into_iter();
    // let value_1 = vec_1_iter.next();

    // for value in vec_1 {
    // for value in &vec_1 {
    for value in &mut vec_1 {
        println!("Value: {}", value);
    }

    let mut person: HashMap<String, i32> = HashMap::new();
    person.insert("Mic".to_string(), 30);
    person.insert("John".to_string(), 25);
    person.insert("Alice".to_string(), 28);

    // for (name, age) in &person {
    // for (name, age) in &mut person {
    for (name, age) in person {
        println!("Name: {}, Age: {}", name, age);
    }
}
