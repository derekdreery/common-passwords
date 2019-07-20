fn main() {
    if common_passwords::is_compromised("password") {
        println!("\"password\" is a compromised password, suprise suprise");
    } else {
        panic!("\"password\" should have been compromised, bugbugbug");
    }
}
