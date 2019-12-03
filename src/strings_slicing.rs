pub fn run() {
    let name_in_urdu = "کامران";

    let s = &name_in_urdu[0..4];
    println!("{}", s);

    // Rust would "Panic" at runtime in the same way as if an invalid index were accessed in a vector
    //let s1 = &name_in_urdu[0..1];
    //println!("{}", s1);

    
}