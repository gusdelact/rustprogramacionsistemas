use std::thread;

static NTHREADS: i32 = 20;

// This is the `main` thread
fn main() {
    let mut children = vec![];

    for i in 0..NTHREADS {
        children.push(thread::spawn(move || {
            println!("hilo con numero {}", i);
        }));
    }

    for child in children {
        let _ = child.join();
    }
}
