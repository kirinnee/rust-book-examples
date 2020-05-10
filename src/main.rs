use core::slice;

extern "C" {
    fn abs(input: i32) -> i32;
}
fn main() {
    let mut arr = [
        "a".to_string(),
        "ab".to_string(),
        "abc".to_string(),
        "abcde".to_string(),
        "abcdef".to_string(),
    ];

    let ptr = arr[..].as_mut_ptr();

    unsafe {
        for i in 0..5 {
            let cptr = ptr.add(i);
            println!("Address = {:?}, value = {:?}", cptr, *cptr);
        }
    }

    let mut x = 5;
    unsafe {
        let x1 = &mut x as *mut i32;
        *x1 = 1234;
        let x2 = x1.add(1);
        *x2 = 12345;
        let x3 = x2.add(1);
        *x3 = 123456;
    }

    let slice = unsafe {
        let r = &mut x as *mut i32;
        slice::from_raw_parts_mut(r, 3)
    };
    println!("slice: {:?}", slice);

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}


