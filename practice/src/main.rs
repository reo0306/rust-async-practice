use std::thread;

fn main() {
    let mut threads = Vec::new();

    for i in 0..8 {
        let handle = thread::spawn(move || {
            let result = fibonaci(4000);
            println!("Thread {} result: {}", i, result);
        });
        threads.push(handle);
    }
    for handle in threads {
        handle.join().unwrap();
    }
}

fn fibonaci(n: u64) -> u64 {
    if n == 0 || n == 1 {
        return n;
    }
    fibonaci(n - 1) + fibonaci(n - 2)
}