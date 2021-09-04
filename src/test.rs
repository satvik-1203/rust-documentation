#[derive(Debug)]
struct Person {
  name: String,
}

pub fn run() {
  let satvik: Person = Person {
    name: String::from("satvik"),
  };

  let copySatvik = &satvik;

  // copySatvik.name = String::from("newName");

  println!("{:#?}", copySatvik);
}
