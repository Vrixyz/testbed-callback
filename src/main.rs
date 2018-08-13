pub struct Data {
  pub data: String,
}

fn main() {
  let data = Data { data: "test" };
  let testbed = Testbed::new(World::new_empty());
  testbed.add_callback( move |_,_,_| {
    println!("{}", data.data);
  }
  testbed.run();
}
