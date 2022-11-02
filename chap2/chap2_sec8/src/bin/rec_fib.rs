fn main() {
    for i in 2..=20 {
        println!("{}", fib(i));
    }
}

fn fib(n: i32) -> i32 {
    if n == 1 {
        return 0;
    }
    if n == 2 {
        return 1;
    }
    return fib(n - 2) + fib(n - 1);
}
