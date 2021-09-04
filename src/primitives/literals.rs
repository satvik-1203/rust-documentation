pub fn run() {
  // addition
  println!("1 +1 = {0}", 1u32 + 1);
  println!("1 - 2 = {}", 1i32 - 2);
  // short circuiting
  println!("{}", false && true);
  println!("{}", true || false);
  // underscore for better readability
  println!("{}", 1_000_000i32);
}
