use std::mem;
use std::borrow::Cow;
use std::ffi::CStr;
use std::os::raw::c_char;

static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];
static mut E: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static mut F: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

fn main() {
    pointer();
    println!("***************************");
    parse();
    println!("***************************");

    unsafe {
        println!("E location: {:p}, value: {:?}", &E, E);
        let mut ptr = (&E as *const u8) as *mut u8;
        println!("ptr = {:p}", ptr);
        
        for i in 0..10 {
            let p = ptr as usize + i;
            *(p as *mut u8) = 10 + i as u8;
        }
        println!("{:?}\n", E);

        println!("F location: {:p}, value: {:?}", &F, F);
        ptr = (&F as *const u8) as *mut u8;
        println!("ptr = {:p}", ptr);

        for i in 0..11 {
            let p = ptr as usize + i;
            *(p as *mut u8) = 100 + i as u8;
        }
        println!("{:?}", F);
    }
    
}


fn pointer() {
    let a = 42;
    let b = &B;
    let c = Box::new(C);

    println!("a (an unsigned integer):");
    println!("  location: {:p}", &a);
    println!("  size:     {} bytes", mem::size_of::<usize>());
    println!("  value:    {}", a);
    println!();

    println!("b (a reference to B):");
    println!("  location: {:p}", &b);
    println!("  size:     {} bytes", mem::size_of::<&[u8; 10]>());
    println!("  pointer to: {:p}", b);
    println!();

    println!("c (a \"box\" for C):");
    println!("  location: {:p}", &c);
    println!("  size:     {} bytes", mem::size_of::<Box<[u8; 11]>>());
    println!("  pointer to: {:p}", c);
    println!();

    println!("B (an arrary of 10 bytes):");
    println!("  location: {:p}", &B);
    println!("  size: {} bytes", mem::size_of::<[u8; 10]>());
    println!("  value: {:?}", B);
    println!();

    println!("C (an arrary of 11 bytes):");
    println!("  location: {:p}", &C);
    println!("  size: {} bytes", mem::size_of::<[u8; 11]>());
    println!("  value: {:?}", C);
    println!();
}


fn parse() {
    let a = 42;
    let b: String;
   let c: Cow<str>;

    unsafe {
        let b_ptr = (&B as *const u8) as *mut u8;
        //b = String::from_raw_parts(b_ptr, 10, 10);
        //b = String::from_raw_parts(E.as_mut_ptr(), E.len(), 10);

        let c_ptr = (&C as *const u8) as *const c_char;
        c = CStr::from_ptr(c_ptr).to_string_lossy();
    }

    println!("a: {}, c: {}", a, c);
    //println!("a: {}, b: {}, c: {}", a, b, c);
}
