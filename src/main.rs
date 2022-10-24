use std::io;

fn main() {
    println!("This is the Fibonacci numbers app!");
    println!("Type a number:");

    let mut max_num = String::new();
    io::stdin()
        .read_line(&mut max_num)
        .expect("Failed to read line!");
    let max_num: u64 = max_num.trim().parse().expect("Please type a number!");

    let mut first_num: u64 = 0;
    let mut second_num: u64 = 1;

    if max_num > 1 {
        print!("Sequence: ");
        for i in 0..max_num {
            if i < 2 {
                print!("{} ", i);
                continue;
            }

            let next_num: u64 = first_num + second_num;
            print!("{} ", next_num);
            first_num = second_num;
            second_num = next_num;
        }
        println!("\nF({}) = {} + {} = {}", max_num, first_num, second_num, first_num + second_num);
    } else {
        println!("F({}) = {}", max_num, max_num);
    }

    println!("Done, exiting.");
}
