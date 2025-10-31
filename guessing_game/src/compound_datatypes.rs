//const my_global_value: String = String::from("My Constant String");
const MY_CONST_CONST: &str = &"hola";

fn main() {
    let list: [i32; 3] = [1, 2, 3];
    println!("{:?}", list);

    // Array of strings?
    let str_list: [&str; 3] = ["kotlin", "Java", "Elixir"];
    println!("{:?}", str_list);

    //Tuples  -> Store hereterogeneous kinds of datatypes
    let my_tuple: (&str, i32, bool) = ("Kotlin!!!!", 1234, true);
    println!("{:?}", my_tuple);
    print!("{}\n", my_tuple.0);

    // Compound tuple
    let compound_tuple = ("Pantheon", 12, true, [1, 2, 3, 4, 5]);
    println!("{:?}", compound_tuple);
    println!("{}", compound_tuple.1);

    let my_str: String = "Kotlin is great".to_string();
    println!("{}", my_str);

    let my_str_slice: &str = &my_str[0..];
    println!("{}", my_str_slice);
    println!("{}", MY_CONST_CONST)
}
