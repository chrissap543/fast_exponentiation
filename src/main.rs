use std::io; 

fn main() {
    println!("Enter the base: "); 
    let mut base = String::new(); 
    io::stdin().read_line(&mut base).expect("Failed to read line"); 
    let base = base.trim().parse().expect("Please type a number"); 
    
    println!("Enter the exponent: "); 
    let mut exp = String::new(); 
    io::stdin().read_line(&mut exp).expect("Failed to read line"); 
    let exp = exp.trim().parse().expect("Please type a number"); 

    println!("{}", power(base, exp)); 
}

fn power(n: u32, m: u32) -> u32 {
    if m == 0 {
        return 1;
    }
    let recursive = power(n, m / 2);
    if m % 2 == 0 {
        return recursive * recursive
    }
    recursive * recursive * n
}
