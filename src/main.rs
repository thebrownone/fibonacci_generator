fn fibonacci (n: u32) -> u64 {

    let mut a= 0;
    let mut b = 1;

    for _element in 0..n {
        let c = a + b;
        a = b;
        b = c;
    }

    a
}


fn main() {
    let n = 10;
    let result = fibonacci(n);
    println!("The {}th fibonacci number is {}", n, result);
}