use std::io;

fn main() {
    let price_p = 3_200.0;
    let price_f = 3_000.0;
    let price_a = 2_500.0;
    let price_e = 2_000.0;
    let price_w = 2_500.0;
    let mut total_cost = 0.0;

    println!("\nWelcome to the PAU Cafetaria Ordering System! What woukd you like today?");
    loop {
        println!("\n------------------------------------");
        println!("\n           Menu                 Price
                  \nP = Poundo Yam/Edinkaiko Soup   3,200
                  \nF = Fried Rice & Chicken        3,000
                  \nA = Amala & Ewedu Soup          2,500
                  \nE = Eba & Egusi Soup            2,000
                  \nW = White Rice & Stew           2,500");
        println!("\n------------------------------------");

        println!("\nPlease select 'add' to add a new item or 'exit' to checkout.");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim();   

        if input == "exit" {
            println!("\nSee you same time tomorrow? Goodbye :)");
            break; 
        } else if input != "add" {
            println!("\nYou must select 'y' or 'exit'");
            continue; 
        }
        
        println!("\nPlease enter an item code (P, F, A, E, W):");
        let mut item_code = String::new();
        io::stdin().read_line(&mut item_code).expect("Failed to read input");
        let item_code = item_code.trim().to_uppercase(); 

        println!("\nPlease enter the quantity:");
        let mut qty = String::new();
        io::stdin().read_line(&mut qty).expect("Failed to read input");
        let qty:f64 = qty.trim().parse().expect("Not a valid quantity");

        let mut item_price = 0.0;

        if item_code == "P" {
            item_price = price_p;
        } else if item_code == "F" {
            item_price = price_f;
        } else if item_code == "A" {
            item_price = price_a;
        } else if item_code == "E" {
            item_price = price_e;
        } else if item_code == "W" {
            item_price = price_w;
        } else {
            println!("\n'{}' is not a valid item code. Item not added.", item_code);
        }
        
        if item_price > 0.0 {
            let cost_for_this_item = item_price * qty;
            total_cost += cost_for_this_item; 
            println!("\nAdded {} x {} (₦{:.2}). Subtotal is now: ₦{:.2}",qty, item_code, cost_for_this_item, total_cost);
        }

    println!("\n--- Checkout ---");
    println!("\nYour total cost is: ₦{:.2}", total_cost);


    let final_amount: f64;
    
    if total_cost > 10_000.0 {
        let discount = total_cost * 0.05;
        final_amount = total_cost - discount;
        println!("\nA 5% discount of ₦{:.2} has been applied.", discount);
    } else {
        final_amount = total_cost;
        println!("\nNo discount applied.");
    }

    println!("\n------------------------------------");
    println!("\nYour final amount payable is: ₦{:.2}. Thank you for shopping with us today!", final_amount);
}
}