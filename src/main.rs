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

fn string_pop() {
    let mut string_pop = String::from("rust pop zhongwen!");
    let p1 = string_pop.pop();
    let p2 = string_pop.pop();
    dbg!(p1);
    dbg!(p2);
    dbg!(string_pop);
}

fn string_remove() {
    let mut string_remove = String::from("testremovemethod");
    println!(
        "string_remove takes {} bytes",
        std::mem::size_of_val(string_remove.as_str())
    );
    string_remove.remove(0);
    dbg!(string_remove);
}

fn string_truncate() {
    let mut string_truncate = String::from("testtruncate");
    string_truncate.truncate(3);
    dbg!(string_truncate);
}

fn string_clear() {
    let mut string_clear = String::from("string_clear");
    string_clear.clear();
    dbg!(string_clear);
}

fn string_concatenate() {
    let string_append = String::from("hello ");
    let string_rust = String::from("rust");
    // Auto de-reference && ownership transfer
    // It uses fn add(self, s: &str) -> String, means a fn will takes ownership
    let result = string_append + &string_rust;
    let mut result = result + "!";
    result += "!!!";
    assert_eq!("hello rust!!!!", result);
    // Error:
    // print!("{}", string_append);
}

fn string_format() {
    let s1 = "hello";
    let s2 = String::from("rust");
    let s = format!("{} {}", s1, s2);
    println!("formated string -> {}", s);
}

fn string_escape() {
    let byte_escape = "I'm writing \x52\x75\x73\x74";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE_STRUCK CAPITAL R\"";
    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );
    let long_string = "String literals
can span multiple lines.
The linebreaker and indentation here ->\
<- cna be escaped too!";
    println!("{}", long_string);
}

fn raw_string() {
    println!("{}", "hello \\x52\\x75\\x73\\x74");

    let raw_string = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_string);

    let quetes = r#"And I said: "There is no escape!""#;
    println!("{}", quetes);

    let long_delimiter = r###"A string with "# in it. And  even "##"###;
    println!("{}", long_delimiter);
}

fn unicode_loop() {
    for c in "Japanese".chars() {
        println!("{}", c);
    }
}

fn bytes_loop() {
    for b in "Japanese".bytes() {
        println!("{}", b);
    }
}

fn main() {
    string_push_ins();
    string_replace();
    string_pop();
    string_remove();
    string_truncate();
    string_clear();
    string_concatenate();
    string_format();
    string_escape();
    raw_string();
    unicode_loop();
    bytes_loop();
}
