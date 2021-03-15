use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::Relaxed;

struct Multiwrite{
    vec: Vec<AtomicU32>,
    slice1: &'static [AtomicU32],
    slice2: &'static mut [AtomicU32],
    clone: bool,
    thread: Option<std::thread::JoinHandle<()>>
    
}

impl Multiwrite{
    fn new(size:usize, init_value: u32) -> Self{
        let mut vec = Vec::with_capacity(size);
        vec.resize_with(size, ||AtomicU32::new(init_value));
        let ptr = vec.as_mut_ptr();
        unsafe{
            Multiwrite{
                vec,
                slice1: std::slice::from_raw_parts(ptr, size),
                slice2: std::slice::from_raw_parts_mut(ptr, size),
                clone: false,
                thread: None
            }
        }
    }

    fn get(&self, idx: usize) -> u32{
        self.slice1[idx].load(Relaxed)
    }
    fn set(&mut self, idx: usize, val: u32){
        self.slice2[idx].store(val, Relaxed);
    }

    fn spawn(& mut self){
        let size = self.slice1.len();
        unsafe{
            let ptr = self.vec.as_mut_ptr();
                
            let mut cloned = Multiwrite{
                vec: Vec::new(),
                slice1: std::slice::from_raw_parts(ptr, size),
                slice2: std::slice::from_raw_parts_mut(ptr, size),
                clone: true,
                thread: None
            };
            self.thread = Some(std::thread::spawn(move ||{
                let mut cnt: u32 = 0;
                loop{
                    cloned.set(0, cnt);
                    cnt +=1;
                }
            }))
        }
        
    }
}



impl Drop for Multiwrite{
    fn drop(&mut self){
        if self.clone{
            loop{}
        }
        if let Some(thread) = &self.thread {
            println!("Other thread found, terminating.");
            // std::process::exit(0);
        }
    }
}

fn main() {
    println!("Hello, world!");
    let mut a = Multiwrite::new(1, 0);
    a.spawn();
    loop{
        println!("{}", a.get(0));
    }
}

#[test]
fn test_create(){
    let a = Multiwrite::new(1,0);
    assert_eq!(a.get(0), 0);
}
#[test]
fn test_write(){
    let mut a = Multiwrite::new(1,0);
    a.set(0, 42);
    assert_eq!(a.get(0), 42);
}