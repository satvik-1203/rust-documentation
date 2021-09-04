// #[derive(Debug)]
// debug module use used for fancy printing.
struct Person<'a> {
  name: String,
  age: u32,
  address: &'a str,
}

pub fn run() {
  let satvik = Person {
    name: String::from("satvik"),
    age: 18,
    address: "hyd",
  };
  // println!("{:#?}", satvik)
  println!("{}", satvik.name)
}
