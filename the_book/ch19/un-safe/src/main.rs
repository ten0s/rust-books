unsafe fn dangerous() {}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    use std::slice;

    let len = slice.len();
    let ptr = slice.as_mut_ptr(); // : *mut i32

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid),
        )
    }
}

extern "C" {
    fn abs(n: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Called a Rust function from C!");
}

static HELLO_WORLD: &str = "Hello, world!";

// accessing and modifying mutable static vars is unsafe
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// 4. Implement an unsafe trait

unsafe trait Foo {}

unsafe impl Foo for i32 {}

fn main() {
    // 1. Dereferencing a Raw Pointer
    let mut num = 5;

    // NB: both immutable and mutable raw pointers at the same time!
    let r1 = &num as *const i32;
    // num must be `mut` to change
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("num is :{}", num);
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        *r2 = 10;
        println!("num is :{}", num);
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    /*
       let address = 0x012345usize;
       let r = address as *const i32;

       unsafe {
           // causes segfault
           println!("r is: {}", *r);
       }
    */

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = split_at_mut(r, 3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // 2. Calling an Unsafe Function or Method

    unsafe {
        dangerous();
    }

    let n = -1;
    unsafe {
        println!("C: abs({}) = {}", n, abs(n));
    }

    // 3. Access or modify a mutable static variable

    println!("name: {}", HELLO_WORLD);

    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    // 5. Access fields of unions
    // https://doc.rust-lang.org/reference/items/unions.html

    #[repr(C)]
    union MyUnion {
        f1: u32,
        f2: f32,
    }

    let u = MyUnion { f1: 10 };
    let f = unsafe {
        println!("u.f1 = {}", u.f1);
    };

    unsafe {
        match u {
            MyUnion { f1: 10 } => {
                println!("ten");
            }
            MyUnion { f2 } => {
                println!("{}", f2);
            }
        }
    }
}
