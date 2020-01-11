use crossbeam_channel::{unbounded, bounded};
use std::time::{Duration, Instant};
use std::thread;
use core_affinity::{self, CoreId};

fn test_unbounded(c1: CoreId, c2: CoreId) {
    core_affinity::set_for_current(c2);
    let (tx, rx) = unbounded();
    thread::spawn(move || {
        core_affinity::set_for_current(c1);
        let start = Instant::now();
        let max = Duration::from_secs(1);
        let mut n  = 0;
        while start.elapsed() < max {
            n += 1;
            tx.send(n);

        }
    });

    let mut count = 0u64;
    for n in rx {
        count = n;
    }
    println!("U {} - {}: {}us/send", c1.id, c2.id, 10000000000 / count);
}

fn test_bounded(c1: CoreId, c2: CoreId) {
    core_affinity::set_for_current(c2);

    let (tx, rx) = bounded(64);
    thread::spawn(move || {
        core_affinity::set_for_current(c1);
        let start = Instant::now();
        let max = Duration::from_secs(1);
        let mut n  = 0;
        while start.elapsed() < max {
            n += 1;
            tx.send(n);

        }
    });

    let mut count = 0u64;
    for n in rx {
        count = n;
    }

    
    println!("B {} - {}: {}us/send", c1.id, c2.id, 10000000000 / count);

}

fn main() {
    let cores = core_affinity::get_core_ids().unwrap();
    for c1 in cores.clone() {
        for c2 in cores.clone() {
            if c2.id > c1.id {
                test_bounded(c1, c2);
            }
        }
    }
    for c1 in cores.clone() {
        for c2 in cores.clone() {
            if c2.id > c1.id {
                test_unbounded(c1, c2);
            }
        }
    }

}
