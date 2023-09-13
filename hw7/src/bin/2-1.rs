fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Invalid input");
        return;
    }
    let mut numbers: Vec<i32> = Vec::new();
    for arg in &args[1..] {
        if let Ok(num) = arg.parse() {
            numbers.push(num);
        }
    }
    if numbers.is_empty() {
        println!("No valid numbers provided.");
        return;
    }


    let paired_numbers =paired_num(&numbers);

    let mut ascending_order = paired_numbers.clone();
    ascending_order.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    println!("Ascending order by X: {:?}", ascending_order);

    let mut ascending_order = paired_numbers.clone();
    ascending_order.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
    println!("Ascending order by Y: {:?}", ascending_order);

    let mut descending_order = paired_numbers.clone();
    descending_order.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
    println!("Descending order by X: {:?}", descending_order);

    let mut descending_order = paired_numbers.clone();
    descending_order.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    println!("Descending order by Y: {:?}", descending_order);
}


fn paired_num(list: &[i32]) -> Vec<(f64, f64)> {
    let mut result: Vec<(f64, f64)> = Vec::new();

    let range = if list.len() % 2 != 0 {
        list.len() - 1
    } else {
        list.len()
    };
    
    for i in (0..range).step_by(2) {
        let num1 = list[i] as f64;
        let num2 = list[i + 1] as f64;
        result.push((num1, num2));
    }
    
    result
}