use std::ffi::{c_int};
use std::ptr::NonNull;
use std::boxed::Box;

#[repr(C)]
pub struct ValueFFI {
    tag: c_int,
    val: *mut u8
}

#[repr(C)]
pub enum ValueEnum {
    NULL,
    SomeVal(*mut i32)
}

#[repr(C)]
pub enum BoxEnum {
    NULL,
    SomeVal(Box<i32>)
}



#[no_mangle]
pub extern "C" fn struct_from_julia(val: *mut ValueFFI) {
    unsafe {
        let val_ffi=&*val;
        println!("{}", val_ffi.tag);
        if val_ffi.tag == 0 as c_int {
            println!("{}", *(val_ffi.val as *mut u8));
        }

    }
}

#[no_mangle]
pub extern "C" fn enum_from_julia(val: *mut ValueEnum) {
    unsafe {
        let val_ffi=&*val;
        match val_ffi {
            &ValueEnum::NULL=>{println!("Null");},
            &ValueEnum::SomeVal(val)=>{println!("{}", &*val);}
        }
    }
}

#[no_mangle]
pub extern "C" fn conv_to_box(val: *mut BoxEnum) {
    unsafe {
        let val_ffi=&*val;
        match val_ffi {
            &BoxEnum::NULL=>{println!("Null");},
            &BoxEnum::SomeVal(ref val)=>{println!("{}", &*val);}
        }
    }
}

#[no_mangle]
pub extern "C" fn drop_boxenum(val: *mut BoxEnum) {
    let _=unsafe {Box::from_raw(val)};
}

#[no_mangle]
pub extern "C" fn get_boxenum()->*mut BoxEnum {
    Box::into_raw(Box::new(BoxEnum::SomeVal(Box::new(99))))
}

#[repr(C)]
pub struct SomeTest {
    x: (i32, i32)
}

#[no_mangle]
pub extern "C" fn give_sometest()->*mut SomeTest {
    Box::into_raw(Box::new(SomeTest{x: (24,42)}))
}