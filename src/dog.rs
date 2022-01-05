struct Dog {
    name: String,
}
impl Dog {
    fn bark(&self) {
        println!("Woof! My name is {}".self.name);
    }
}
