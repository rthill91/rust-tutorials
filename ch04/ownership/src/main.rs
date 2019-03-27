fn main() {
    let s = String::from("hello");
    let s = takes_ownership(s);
    println!("{} again", s);
}

fn takes_ownership(some_string: String) -> String {
    println!("{}", some_string);

    some_string
}
