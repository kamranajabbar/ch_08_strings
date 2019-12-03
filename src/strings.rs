pub fn run(){
    let estr = String::new();                   // Empty String
    println!("Empty String : {:?}", estr);

    let data = "Kamran Jabbar";                 // String Literals
    let s = data.to_string();
    println!("String 1 : {:?}", s);

    let s = String::from("initial contents");   // Create a String from a string literal
    println!("String 2 : {:?}", s);

    let salam = String::from("ﻢﻜﯿﻠﻋ مﻼﺴﻟا");
    println!("String 3 UTF-8 : {:?}", salam);
}