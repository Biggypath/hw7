use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Invalid input");
        return;
    }
    
    let mut numbers = Vec::new();

    for arg in args.iter().skip(1) {
        match arg.parse::<i32>() {
            Ok(num) => {
                numbers.push(num);
            }
            Err(_) => {
                println!("Invalid input: {}", arg);
            }
        }
    }

    if numbers.is_empty() {
        println!("No valid numbers provided.");
        return;
    }

    numbers.sort();
    println!("Ascending order: {:?}", numbers);

    numbers.sort_by(|a, b| b.cmp(a));
    println!("Descending order: {:?}", numbers);
}



