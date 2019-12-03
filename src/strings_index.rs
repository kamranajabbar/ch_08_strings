pub fn run() {
    // let s = String::from("ALLAH");
    // let new_string = &s[0];
    // println!("String : {:?}", new_string);

    // let slit = "ALLAH";
    // let new_string1 = &slit[0];
    // println!("String : {:?}", new_string1);

    // let hello = "Здравствуйте"
    // let answer = &hello[0];
    // println!("String : {:?}", answer);

    let len1 = String::from("Hola").len();          //In UTF-8, Each letters takes 1 byte
    let len2 = String::from("Здравствуйте").len();  //In UTF-8, Each Unicode scalar value takes 2 bytes
    let len3 = String::from("کامران").len();

    println!("{}", len1);
    println!("{}", len2);
    println!("{}", len3);
}