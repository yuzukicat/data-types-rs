fn deconstructing_tuples_with_matching_patterns() {
    let tup = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {}", y);
}

fn use_dot_to_access_the_tuple() {
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let _five_hundred = x.0;
    let _six_point_four = x.1;
    let _one = x.2;
}

fn returning_multiple_values_with_a_tuple() {
    let s1 = String::from("Hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn main() {
    deconstructing_tuples_with_matching_patterns();
    use_dot_to_access_the_tuple();
    returning_multiple_values_with_a_tuple();
}
