fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];

    let mut largest = input[0];
    let mut smallest = input[0];

    for &number in input.iter() {
        if number > largest {
            largest = number;
        }
        if number < smallest {
            smallest = number;
        }
    }

    println!("{} is largest and {} is smallest", largest, smallest);
}

