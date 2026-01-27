fn main() {
    let mut s = String::from("Hi");
    let r1 = &s;
    let r2 = &s;
    println!("r1 = {r1} / r2 = {r2}");
    println!("again, r1 = {r1}");

    let r3 = &mut s;
    println!("r3 = {r3}");

    let s = String::from("Yoyi Guiller");
    let w = first_word(&s);
    println!("first word = {w}");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
