pub fn run(){
    let mut name = String::from("Kamran");
    name.push_str(" Jabbar");
    println!("My name is -> {:?}", name);

    let mut pakistan = String::from("Pakistan");
    let zindabad = " Zindabad";
    pakistan.push_str(zindabad);
    println!("Sab mil kar bolo -> {:?}", pakistan);
}