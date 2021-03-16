use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::Arc;

struct Multiwrite {
    data:  Arc<[AtomicU32]>,
    thread: Option<std::thread::JoinHandle<()>>,
}

impl Multiwrite{
    fn new(size: usize, init_value: u32) -> Result<Self, &'static str> {
        if size == 0{
            return Err("size should be > 0");
        }
        let mut vec = Vec::with_capacity(size);
        vec.resize_with(size, || AtomicU32::new(init_value));
        Ok(Multiwrite {
                data: Arc::from(vec),
                thread: None,
        })
    }

    fn get(&self, idx: usize) -> u32 {
        self.data[idx].load(Relaxed)
    }
    fn set(&self, idx: usize, val: u32) {
        self.data[idx].store(val, Relaxed);
    }

    // fn copy_into_slice<T>(&self, dest: &mut [T]) -> Result<(), &'static str>
    // where T: bytemuck::Pod {
    //         if self.slice.len() * std::mem::size_of_val(&self.slice[0]) != dest.len() * std::mem::size_of_val(&dest[0]){
    //             return Err("Size of target should be the same as source.")
    //         }
    //         let target_u32: &mut [u32] = bytemuck::cast_slice_mut(dest);
    //         for idx in 0..self.slice.len(){
    //             unsafe {
    //                 *target_u32.get_unchecked_mut(idx) = self.slice.get_unchecked(idx).load(Relaxed);
    //             }
    //         }
    //         Ok(())
    // }

    fn spawn<F>(&mut self, f: F)
    where
        F: FnOnce(Multiwrite) + Clone + Send + 'static,
    {
        let cloned = Multiwrite {
                data: self.data.clone(),
                thread: None,
            };
        self.thread = Some(std::thread::spawn(move || f(cloned)));
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

fn loop_set(mw: Multiwrite) {
    let mut cnt: u32 = 0;
    loop {
        mw.set(0, cnt);
        cnt += 1;
    }
}

fn main() {
    let mut a = Multiwrite::new(1, 0).unwrap();
    a.spawn(loop_set);
    for i in 0..10 {
        while a.get(0) < 2147483648 {
        }
        println!("{} lower part done", i);
        while a.get(0) > 65536 {
        }
        println!("{} upper part done", i);
    }
}

#[test]
fn test_create() {
    let a = Multiwrite::new(1, 0).unwrap();
    assert_eq!(a.get(0), 0);
}
#[test]
fn test_write() {
    let mut a = Multiwrite::new(1, 0).unwrap();
    a.set(0, 42);
    assert_eq!(a.get(0), 42);
}
