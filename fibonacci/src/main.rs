use std::io;

fn main() {
    println!("Enter fibonacci number: ");

    let mut num = String::new();
    
    io::stdin().read_line(&mut num)
        .expect("Failed read line.");

    let num: usize = num.trim().parse()
        .expect("Please enter a positive integer.");

    fibonacci(num);
}

fn fibonacci(n: usize) {
    let mut fib = vec![1; n];

    for i in 2..fib.len() {
        fib[i] = fib[i - 1] + fib[i - 2];
    }

    println!("This is first {} fibonacci numbers:", n);
    println!("{:?}", fib);

}
