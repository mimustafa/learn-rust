use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let count = Arc::new(Mutex::new(0_u32));
    let mut workers = Vec::new();

    for _ in 0..2 {
        let count_for_worker = Arc::clone(&count);
        workers.push(thread::spawn(move || {
            // The mutex allows only one thread at a time to change the shared counter.
            let mut value = count_for_worker
                .lock()
                .expect("counter mutex is not poisoned");
            *value += 1;
        }));
    }

    for worker in workers {
        worker.join().expect("worker did not panic");
    }

    println!(
        "Counter after both workers: {}",
        *count.lock().expect("counter mutex is not poisoned")
    );
    println!("Arc shares ownership; the Mutex provides the synchronized mutation.");
}
