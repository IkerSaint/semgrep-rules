#[derive(Debug, Default, Copy, Clone)]
#[repr(C, packed)]
struct MyStruct {
    aligned: u8,
    unaligned: u32,
}

fn main() {
  let a = 5;
  let raw_a = &a as *const i32;

  let mut b = 10;
  let raw_b_mut = &mut b as *mut i32;

  let c: i32 = 10;
  let raw_c: *const i32 = &c;

  let mut d: i32 = 42;
  let raw_d_mut: *mut i32 = &mut d;

  let my_box: Box<i32> = Box::new(42);
  let my_raw_box: *mut i32 = Box::into_raw(my_box);

  let st = MyStruct::default();
  let pt = std::ptr::addr_of!(st.unaligned);

  let e = 0x012345usize;
  let raw_e = e as *const i32;

  let s: &str = "123";
  let ptr = s.as_ptr();
}