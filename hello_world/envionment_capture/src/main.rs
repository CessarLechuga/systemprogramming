fn track_changes() {
    let mut tracker = 0;
    let mut update = || {
        tracker += 1;
        println!("Count: {}", tracker);
    };

    update(); // Output: Count: 1
    update(); // Output: Count: 2
}

fn main() {
    track_changes();
}
