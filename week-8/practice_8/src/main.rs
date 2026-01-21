fn main() {
    //initialize a mutable tuple
    let mut mountain_heights = ("Everst", 8848, "fishtail", 6933);

    println!("Original tuple = {:?}", mountain_heights);

    //Change 3rd and 4th element of a mutable tuple
    mountain_heights.3 = "Lhoste";
    mountain_heights.4 = 8516;

    println!("Changed tuple = {:?}",mountain_heights );
}
