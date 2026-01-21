use::std::io::Read;
use::std::io::Write;

fn main() {
    let mut file = std::fs::File::create("Welcome_message.txt").expect("creat failed");
    file.write("David is black..\n".as_bytes());
    let mut file = std::fs::File::open("Welcome_message.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("read failed.");
    println!("{}",contents);
}
