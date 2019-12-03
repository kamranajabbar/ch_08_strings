pub fn run(){
    let name1 = String::from("Kamran");
    let name2 = String::from(" Jabbar");

    println!("Fullname : {:?}", name1+&name2);
    //println!("First name : {:?}", name1);           // name1 is no longer valid after the addition
    println!("Last name : {:?}", name2);



    let s1 = String::from("God");
    let s2 = String::from("Bless");
    let s3 = String::from("You");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("Concatenation of s1,s2,s3 : {:?}", s);



    let st1 = String::from("ALLAH");
    let st2 = String::from("Rahim");
    let st3 = String::from("Karay");

    let my_formated_string = format!("{}-{}-{}", st1, st2, st3);
    println!("Formated Concatenation of st1,st2,st3 : {:?}", my_formated_string);
}