pub fn run() {
    let name_in_urdu = "کامران";

    for chars in name_in_urdu.chars() {
        println!("Chars list in Strings {}", chars);
    }

    println!("\n");

    for bytes in name_in_urdu.bytes() {
        println!("Bytes list in Strings {}", bytes);
    }
}