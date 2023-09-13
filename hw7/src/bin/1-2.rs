fn main() {
    let args: Vec<String> = std::env::args().collect();

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

    bubble_sort_asc(& mut numbers);
    println!("Ascending order: {:?}", numbers);

    bubble_sort(& mut numbers);
    println!("Descending order: {:?}", numbers);
}

fn bubble_sort(arr: &mut Vec<i32>) {
    let n = arr.len();
    
    for i in 0..n - 1 {
        for j in 0..n - i - 1 {
            if arr[j] < arr[j + 1] {
                let temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
            }
        }
    }
}

fn bubble_sort_asc(arr: &mut Vec<i32>) {
    let n = arr.len();
    
    for i in 0..n - 1 {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                let temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
            }
        }
    }
}