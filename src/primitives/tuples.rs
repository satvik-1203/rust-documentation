fn reverse(pair: (i32, bool)) -> (bool, i32) {
  let (integer, boolean) = pair;
  (boolean, integer)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

#[derive(Debug)]
struct Structure(i32);

pub fn run() {
  let (a, b) = reverse((21i32, true));
  println!("{:?}", (a, b));

  let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

  println!("tuple of tuples: {:?}", tuple_of_tuples);

  let matrix = Matrix(1.1, 1.2, 2.1, 2.2);

  println!("{:?}", matrix);
  println!("{:?}", Structure(3));
}
