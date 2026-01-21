use std::io;

fn main() {

   let mut input1 = String::new();
   let mut input2 = String::new();
   let mut input3 = String::new();

    println!("enter coefficient of x^2");
     io::stdin().read_line(&mut input1).expect("not a valid string");
     let a:f32 = input1.trim().parse().expect("not a valid number");

    println!("enter coefficient of x");
     io::stdin().read_line(&mut input2).expect("not a valid string");
     let b:f32 = input2.trim().parse().expect("not a valid number");

    println!("enter ");
     io::stdin().read_line(&mut input3).expect("not a valid string");
     let c:f32 = input3.trim().parse().expect("not a valid number");

    let d:f32 = b * b - 4.0 * a * c;
    let x1 = (-b + d.sqrt())/ (2.0 * a);
    let x2 = (-b - d.sqrt())/ (2.0 * a);
    println!("the roots of the equation are {} and {}", x1 , x2 );

    if d > 0.0
    {
        println!("roots are distinct");
    }else if d < 0.0 {
        println!("roots are imaginary");
    }else if d == 0.0
    {
        println!("roots are equal and real");
    }else {
        println!("math error!");
    }


}

  
