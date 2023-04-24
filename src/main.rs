fn string_push_ins() {
    let mut s = String::from("Hello ");

    s.push_str("rust");
    println!("push_str() -> {}", s);

    s.push('!');
    println!("push -> {}", s);

    s.insert(5, ',');
    println!("insert_str -> {}", s);

    s.insert_str(6, " I like");
    println!("insert_str -> {}", s);
}

fn string_replace() {
    let string_replace = String::from("I like rust, Learning rust is my favorite!");
    let new_string_replace = string_replace.replace("rust", "RUST");
    dbg!(new_string_replace);

    let new_string_replacen = string_replace.replacen("rust", "RUST", 1);
    dbg!(new_string_replacen);

    let mut string_replace_range = String::from("I like rust!");
    string_replace_range.replace_range(7..8, "R");
    dbg!(string_replace_range);
}



fn main() {
    string_push_ins();
    string_replace();
}
