use std::io;

fn main() {
    // --- Define prices *once* at the top ---
    let price_l = 550_000.0;
    let price_m = 120_000.0;
    let price_k = 15_000.0;
    let price_h = 25_000.0;

    // --- FIX 1: Create a total_cost "accumulator" *outside* the loop ---
    // It must be 'mut' so we can add to it.
    let mut total_cost = 0.0;

    println!("\nWelcome to your Computer Store!");

    loop {
        // --- Task 1: Display the menu (inside the loop so user can see it) ---
        println!("\n------------------------------------");
        println!("\nCode        Item        Price
                  \nL           Laptop      550,000
                  \nM           Monitor     120,000
                  \nK           Keyboard    15,000
                  \nH           Headset     25,000");
        println!("\n------------------------------------");

        println!("\nPlease select 'y' to add a new item and 'n' to checkout.");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim();    // Trim the input for comparison

        if input == "n" {
            break; // Exit the loop to go to checkout
        } else if input != "y" {
            println!("\nYou must select 'y' or 'n'");
            continue; // Skip the rest of this loop and start from the top
        }
        
        // --- Task 2: Ask for item code and quantity ---
        println!("\nPlease enter an item code (L, M, K, H):");
        let mut item_code = String::new();
        io::stdin().read_line(&mut item_code).expect("Failed to read input");
        let item_code = item_code.trim().to_uppercase(); // Trim and convert to uppercase

        println!("\nPlease enter the quantity:");
        let mut qty = String::new();
        io::stdin().read_line(&mut qty).expect("Failed to read input");
        let qty:f64 = qty.trim().parse().expect("Not a valid quantity");

        // --- FIX 2: Use 'match' to find the price ---
        // 1. Declare a mutable variable with a default value.
        let mut item_price = 0.0;

        // 2. Use 'if' blocks to assign a new value to it.
        if item_code == "L" {
            item_price = price_l;
        } else if item_code == "M" {
            item_price = price_m;
        } else if item_code == "K" {
            item_price = price_k;
        } else if item_code == "H" {
            item_price = price_h;
        } else {
            println!("'{}' is not a valid item code. Item not added.", item_code);
            // item_price is already 0.0, so we don't need to do anything
        }
        // --- End of the if/else block ---


        // --- Task 3: Add this item's cost to the total_cost ---
        if item_price > 0.0 {
            let cost_for_this_item = item_price * qty;
            total_cost += cost_for_this_item; 
            println!(
                "Added {} x {} (₦{:.2}). Subtotal is now: ₦{:.2}",qty, item_code, cost_for_this_item, total_cost);
        }

    } // --- End of loop ---

    // --- The user pressed 'n', so the loop is broken. Now we check out. ---

    println!("\n--- Checkout ---");
    println!("Your total cost is: ₦{:.2}", total_cost);

    // --- Task 4: Apply 7% discount if total > 500,000 ---
    let final_amount: f64;
    
    if total_cost > 500_000.0 {
        let discount = total_cost * 0.07;
        final_amount = total_cost - discount;
        println!("A 7% discount of ₦{:.2} has been applied.", discount);
    } else {
        final_amount = total_cost;
        println!("No discount applied.");
    }

    // --- Task 5: Display the final amount payable ---
    println!("------------------------------------");
    println!("Your final amount payable is: ₦{:.2}. Thank you for shopping with us today!", final_amount);
}