struct Laptop {
    brand: String,
    price: u32,
}

impl Laptop {
    // method to compute cost for any quantity
    fn cost(&self, quantity: u32) -> u32 {
        self.price * quantity
    }
}

fn main() {
    // create instances of the struct
    let hp = Laptop {
        brand: "HP".to_string(),
        price: 650_000,
    };

    let ibm = Laptop {
        brand: "IBM".to_string(),
        price: 755_000,
    };

    let toshiba = Laptop {
        brand: "Toshiba".to_string(),
        price: 550_000,
    };

    let dell = Laptop {
        brand: "Dell".to_string(),
        price: 850_000,
    };

    // number of laptops customer wants from each brand
    let qty = 3;

    // calculate total cost
    let total_cost =
        hp.cost(qty) +
        ibm.cost(qty) +
        toshiba.cost(qty) +
        dell.cost(qty);

    println!("Total cost for purchasing 3 laptops from each brand = ₦{}", total_cost);
}
