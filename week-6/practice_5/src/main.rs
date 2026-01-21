fn main() {
   let fullname = "Pan-Atlantic University";
    println!("Hello, world!");
    println!("Name: {}", fullname);
    println!();
    println!("Before trim");
    println!("Length is {}", fullname.len());
    println!();
    println!("After trim");
    println!("Length is {}", fullname.trim().len());
}
