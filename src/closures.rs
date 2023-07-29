pub fn what() {
    let mut str = String::from("Hello");

    //this needs to be mutable, because a call inside it is mutable
    let mut _closure = || str.push_str("World!");
    //this doesn't compile because we're borrowing as mutable and immutable at the same time
    // println!("{str}");
    _closure();
}