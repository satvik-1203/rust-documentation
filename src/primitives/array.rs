use std::mem;
// module used for dealing with memory.
// This module contains functions for querying the size and alignment of types, initializing and manipulating memory.

fn analyze_fn(arr: &[i32]) -> (i32, usize) {
  let first_int: i32 = arr[0];
  let length = arr.len();
  (first_int, length)
}

struct Obj<'a> {
  test: &'a str,
}

pub fn run() {
  let xs: [i32; 10] = [0; 10];
  // for initializing an array of size 5 and type should be given before;
  let obj_arr: [Obj; 1] = [Obj { test: "satvik" }];
  // U can also make an array of custom objects with struct.
  let ans = analyze_fn(&xs[0..4]);
  println!("{:?}", ans);
}
