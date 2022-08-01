fn main() {
    println!("Welcome to FizzBuzz in Rust");
    println!("Enter n: ");
    let n: u32 = loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();

        let value: i32 = line.trim().parse().unwrap();
        if value >= 0 {
            break value as u32
        }
    };
    println!("FizzBuzz...");
    fizzbuzz(n);
}

fn fizzbuzz(n: u32) {
    for i in 1..n+1 {
        if i % 3 == 0 || i % 5 == 0 {
            if i % 3 == 0 {
                print!("Fizz");
            }
            if i % 5 == 0 {
                print!("Buzz");
            }
        } else {
            print!("{}", i);
        }
        println!();
    }
}
