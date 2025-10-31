use std::io::WriterPanicked;

fn main() {
    let mut value = String::from("Kotlin!!");
    value.push_str(" is the best");
    takes_ownership(&value);
    print!("{value}"); // -> Give compile error since the value is dropped at the end of the function

    let a = String::from("Jupiter!");
    let a = takes_and_give_back(a);
    print!("{a}"); // Valid code since the latter funcion moves the value of a in the return statement;
}

fn takes_ownership(s: &String) {
    // This function takes takes_ownership of String s
    print!("{s}");
}

fn takes_and_give_back(s: String) -> String {
    s
}

fn print_reference(s: &String) {
    println!("{s}")
}
