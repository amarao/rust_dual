use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::Relaxed;

struct Multiwrite {
    vec: Vec<AtomicU32>,
    slice: &'static mut [AtomicU32],
    thread: Option<std::thread::JoinHandle<()>>,
}

impl Multiwrite {
    fn new(size: usize, init_value: u32) -> Self {
        let mut vec = Vec::with_capacity(size);
        vec.resize_with(size, || AtomicU32::new(init_value));
        let ptr = vec.as_mut_ptr();
        unsafe {
            Multiwrite {
                vec,
                slice: std::slice::from_raw_parts_mut(ptr, size),
                thread: None,
            }
        }
    }

    fn get(&self, idx: usize) -> u32 {
        self.slice[idx].load(Relaxed)
    }
    fn set(&mut self, idx: usize, val: u32) {
        self.slice[idx].store(val, Relaxed);
    }

    fn spawn<F>(&mut self, f: F)
    where
        F: FnOnce(Multiwrite) + Clone + Send + 'static,
    {
        let size = self.slice.len();
        unsafe {
            let ptr = self.vec.as_mut_ptr();

            let mut cloned = Multiwrite {
                vec: Vec::new(),
                slice: std::slice::from_raw_parts_mut(ptr, size),
                thread: None,
            };
            self.thread = Some(std::thread::spawn(move || f(cloned)));
        }
    }
}

impl Drop for Multiwrite {
    fn drop(&mut self) {
        if self.thread.is_some() {
            println!("Other thread is found running, terminating.");
            std::process::exit(0);
        }
    }
}

fn loop_set(mut mw: Multiwrite) {
    let mut cnt: u32 = 0;
    loop {
        mw.set(0, cnt);
        cnt += 1;
    }
}

fn main() {
    println!("Hello, world!");
    let mut a = Multiwrite::new(1, 0);
    a.spawn(loop_set);
    loop {
        println!("{}", a.get(0));
        if a.get(0) >= 65536 {
            break;
        }
    }
}

#[test]
fn test_create() {
    let a = Multiwrite::new(1, 0);
    assert_eq!(a.get(0), 0);
}
#[test]
fn test_write() {
    let mut a = Multiwrite::new(1, 0);
    a.set(0, 42);
    assert_eq!(a.get(0), 42);
}
