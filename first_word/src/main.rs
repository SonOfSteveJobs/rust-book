fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
fn rest_words(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i + 1..];
        }
    }
    &s[..]
}
fn main() {
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("the first word is: {}", word);

    let hello = &s[0..5];
    let world = &s[6..];
    println!("hello: {hello}");
    println!("world: {world}");

    let s2 = String::from("mr. evgeniy has a big penis");
    let rest_words_str = rest_words(&s2);
    println!("the rest words are: {}", rest_words_str);
    println!(
        "the rest words of the rest words are: {}",
        rest_words(&rest_words_str[10..])
    );

    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..3];
    println!("slice: {:?}", slice);
    assert_eq!(slice, &[2, 3]);
}
