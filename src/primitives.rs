pub mod array;
pub mod literals;
pub mod tuples;

pub fn run() {
  let boolean: bool = true;
  // denoting a boolean

  let float: f32 = 3.0;
  // denoting a float, default f32

  let int: i32 = 2;
  // denoting a number, default i32

  let mut inferred_type = 12;
  inferred_type = 48718947189412897i64; // type inferred by i64
}
