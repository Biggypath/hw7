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

    let paired_numbers = paired_num(&numbers);

    let mut ascending_order = paired_numbers.clone();
    bubble_sort_asc_by_x(&mut ascending_order);
    println!("Ascending order by X: {:?}", ascending_order);

    bubble_sort_asc_by_y(&mut ascending_order);
    println!("Ascending order by Y: {:?}", ascending_order);

    let mut descending_order = paired_numbers.clone();
    bubble_sort_by_x(&mut descending_order);
    println!("Descending order by X: {:?}", descending_order);

    bubble_sort_by_y(&mut descending_order);
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

fn bubble_sort_by_x(arr: &mut Vec<(f64, f64)>) {
    let n = arr.len();

    for i in 0..n - 1 {
        for j in 0..n - i - 1 {
            if arr[j].0 < arr[j + 1].0 {
                let temp = arr[j].0;
                arr[j].0 = arr[j + 1].0;
                arr[j + 1].0 = temp;
            }
        }
    }
}

fn bubble_sort_asc_by_x(arr: &mut Vec<(f64, f64)>) {
    let n = arr.len();

    for i in 0..n - 1 {
        for j in 0..n - i - 1 {
            if arr[j].0 > arr[j + 1].0 {
                let temp = arr[j].0;
                arr[j].0 = arr[j + 1].0;
                arr[j + 1].0 = temp;
            }
        }
    }
}

fn bubble_sort_by_y(arr: &mut Vec<(f64, f64)>) {
    let n = arr.len();

    for i in 0..n - 1 {
        for j in 0..n - i - 1 {
            if arr[j].1 < arr[j + 1].1 {
                let temp = arr[j].1;
                arr[j].1 = arr[j + 1].1;
                arr[j + 1].1 = temp;
            }
        }
    }
}

fn bubble_sort_asc_by_y(arr: &mut Vec<(f64, f64)>) {
    let n = arr.len();

    for i in 0..n - 1 {
        for j in 0..n - i - 1 {
            if arr[j].1 > arr[j + 1].1 {
                let temp = arr[j].1;
                arr[j].1 = arr[j + 1].1;
                arr[j + 1].1 = temp;
            }
        }
    }
}
