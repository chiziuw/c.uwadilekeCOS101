fn main() {
    // lets state their prices
    let toshiba = 450000.0;
    let mac = 1500000.0;
    let hp = 750000.0;
    let dell = 2850000.0;
    let acer = 250000.0;

    // lets state their quanitites
    let toshiba_quant = 2.0;
    let mac_quant = 1.0;
    let hp_quant = 3.0;
    let dell_quant = 3.0;
    let acer_quant = 1.0;

    // lets state the formula to get their total sum for each type
    let toshiba_sum = toshiba * toshiba_quant;
    let mac_sum = mac * mac_quant;
    let hp_sum = hp * hp_quant;
    let dell_sum = dell * dell_quant;
    let acer_sum = acer * acer_quant;

    // lets state the formula to get the total sum, quanitity and average for everything
    let total_sum = toshiba_sum + mac_sum + hp_sum + dell_sum + acer_sum;
    let total_quant = toshiba_quant + mac_quant + hp_quant + dell_quant + acer_quant;
    let average = total_sum / total_quant;

    println!("Total Sum is ₦{} and the average is ₦{}.",total_sum,average);
}