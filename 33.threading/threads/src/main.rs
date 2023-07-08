fn main() {
    let handle = std::thread::spawn(|| {
        for i in 0..10 {
            println!("spawned: {i}");
            std::thread::yield_now();
        }
    });

    for i in 0..10 {
        println!("main: {i}");
        std::thread::yield_now();
    }

    handle.join().unwrap();
}
