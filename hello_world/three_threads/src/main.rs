use std::thread;

fn spawn_threads() {
    let mut handles = vec![];
    
    for i in 1..=3 {
        let handle = thread::spawn(move || {
            println!("Thread {}", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("All threads completed.");
}

fn main() {
    spawn_threads();
}
