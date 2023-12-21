#[repr(C)]
struct Test {
    x: *mut u8
}

fn main() {
    let mut a: &mut [u8]=&mut [1,2,3,4].into_iter().map(|x| x as u8).collect::<Vec<u8>>();
    let test=Test{x: a as *mut [u8] as *mut u8};
    let something=unsafe {std::slice::from_raw_parts(test.x, 4)};
    println!("{:?}", something);
}

// https://stackoverflow.com/questions/53458784/why-is-casting-a-const-reference-directly-to-a-mutable-reference-invalid-in-rust
// https://stackoverflow.com/questions/27150652/how-can-i-get-an-array-or-a-slice-from-a-raw-pointer
// https://stackoverflow.com/questions/39785597/how-do-i-get-a-slice-of-a-vect-in-rust