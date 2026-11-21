use std::{sync::mpsc, thread};

fn main() {
    let (sender, receiver) = mpsc::channel();

    let worker = thread::spawn(move || {
        // The channel transfers this owned number from the worker to the receiver.
        sender.send(6 * 7).expect("receiver is still open");
    });

    println!(
        "Worker result: {}",
        receiver.recv().expect("worker sent a value")
    );
    worker.join().expect("worker did not panic");
}
