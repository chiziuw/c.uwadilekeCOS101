struct LaptopPrice{
    hp_laptops:u32,
    ibm_laptops:u32,
    toshiba_laptops:u32,
    dell_laptops:u32
}

impl LaptopPrice{
    fn total_cost(&self)->u32{
        3*self.hp_laptops + 3*self.ibm_laptops + 3*self.toshiba_laptops + 3*self.dell_laptops
    }
}



fn main() {
    let shop = LaptopPrice{
        hp_laptops:650000,
        ibm_laptops:755000,
        toshiba_laptops:550000,
        dell_laptops: 850000
    };
    println!("The total cost of buying 3 of each brand is: N{}", shop.total_cost());
}