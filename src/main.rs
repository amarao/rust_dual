use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::Relaxed;

struct Multiwrite{
    slice1: &'static [AtomicU32],
    slice2: &'static mut [AtomicU32],
    len: usize
}

impl Multiwrite{
    fn new(size:usize, init_value: u32) -> Self{
        let mut vec = Vec::with_capacity(size);
        vec.resize_with(size, ||AtomicU32::new(init_value));
        let slice = vec.as_mut_slice();
        let slice1 = slice.as_ptr();
        let slice2 = slice.as_mut_ptr();
        unsafe{
            Multiwrite{
                slice1: std::slice::from_raw_parts(slice1, size),
                slice2: std::slice::from_raw_parts_mut(slice2, size),
                len: size
            }
        }
    }

    fn get(&self, idx: usize) -> u32{
        if idx >= self.len{
            panic!("Out of boundry access");
        }
        self.slice1[idx].load(Relaxed)
    }
    fn set(&mut self, idx: usize, val: u32){
        if idx >= self.len{
            panic!("Out of boundry access");
        }
        self.slice2[idx].store(val, Relaxed);
    }
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_create(){
    let a = Multiwrite::new(1,0);
    assert_eq!(a.get(0), 0);
}

fn test_write(){
    let mut a = Multiwrite::new(1,0);
    a.set(0, 42);
    assert_eq!(a.get(0), 42);
}