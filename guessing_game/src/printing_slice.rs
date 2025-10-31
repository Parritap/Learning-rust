use std::fmt::Debug;

// Slices are kind of references, so they do not allow for borrowing.
pub fn main() {
    //let list: &[&str] = &[&"hola", &"mundo", "kotlin".to_string().as_str()];
    //let list = &["hola".to_string(), "kotlin".to_string()];
    let mut phrase: String = String::from("Maria cruza la calle");
    let first_word = get_first_word(&phrase);
    println!("{}", first_word);
    phrase.clear();
    println!("{}", first_word);

    //print_collections(list);
}

fn get_first_word_index(phrase: &String) -> usize {
    let bytes = phrase.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    phrase.len()
}

fn get_first_word(phrase: &String) -> &str {
    let n = get_first_word_index(phrase);
    &phrase[0..n]
}

fn print_collections<Foo: Debug>(collection: &[Foo]) {
    println!("Printing collection!");

    for e in collection {
        println!("{:?} ", e);
    }
}
