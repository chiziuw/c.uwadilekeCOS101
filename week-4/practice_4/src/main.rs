use std::io;

fn main()
{
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    println!("Enter the value of a: ");
    io::stdin().read_line(&mut a).expect("Not a valid string");
    let a:i32 = a.trim().parse().expect("Not a valid number");

    println!("Enter the value of b: ");
    io::stdin().read_line(&mut b).expect("Not a valid string");
    let age:i32 = b.trim().bparse().expect("Not a valid number");

    println!("Enter the value of c: ");
    io::stdin().read_line(&mut c).expect("Not a valid string");
    let age:i32 = c.trim().parse().expect("Not a valid number");

    let root1 = (-b + sqrt(b.powf(2) - 4 * a * c)) / (2 * a)
    let root2 = (-b - sqrt(b.powf(2) - 4 * a * c)) / (2 * a)

    println!("root 1 is {} and root 2 is {}", root1, root2);
}