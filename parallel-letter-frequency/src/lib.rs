// https://docs.rs/crossbeam/latest/crossbeam/
use crossbeam::channel as cb_channel;
use std::collections::HashMap;
use std::sync::mpsc::{self, SyncSender};
use std::thread::{self, JoinHandle};

// We create two channels, one for sending data to the workers, and one for
// receiving from them.
pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    // https://doc.rust-lang.org/std/sync/mpsc/fn.sync_channel.html
    let (tx1, rx1) = mpsc::sync_channel(worker_count);
    let (tx2, rx2) = cb_channel::bounded(input.len());

    let mut workers = Vec::new();
    // Create one less worker, so that we don't end up cloning more than needed.
    // Without this, the program hangs. For channel disconnection to work,
    // every reference and clone must go out of scope.
    //
    // An alternative is to create one more clone and close the originals immediately.
    for i in 0..worker_count - 1 {
        workers.push(worker(i, rx2.clone(), tx1.clone()));
    }
    // The final worker gets the original references
    workers.push(worker(worker_count - 1, rx2, tx1));

    // Send all lines and close the channel
    for line in input {
        tx2.send(line.to_string())
            .unwrap_or_else(|_| panic!("[main] Failed to send"));
    }
    drop(tx2);

    let mut freq = HashMap::new();
    // Loop until channel is disconnected
    for map in rx1 {
        for (c, i) in map {
            *freq.entry(c).or_default() += i;
        }
    }
    // Make sure there are no orphan threads
    for w in workers {
        w.join().unwrap_or_else(|_| panic!("[main] Failed to join"));
    }
    freq
}

fn worker(
    id: usize,
    rx: cb_channel::Receiver<String>,
    tx: SyncSender<HashMap<char, usize>>,
) -> JoinHandle<()> {
    thread::spawn(move || loop {
        let mut hm = HashMap::new();
        // This method will never block the caller in order to wait for
        // data to become available.
        match rx.try_recv() {
            Ok(line) => {
                for c in line.as_str().chars() {
                    if c.is_alphabetic() {
                        for x in c.to_lowercase() {
                            *hm.entry(x).or_default() += 1;
                        }
                    }
                }
                tx.send(hm)
                    .unwrap_or_else(|_| panic!("worker-[{}] failed to send", id));
            }
            Err(e) if e == cb_channel::TryRecvError::Disconnected => break,
            _ => {}
        }
    })
}
