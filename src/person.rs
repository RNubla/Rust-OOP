pub struct Person {
    pub first_name: String,
    pub last_name: String,
    pub age: i32,
}

impl Person {
    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
    pub fn details(&self) -> String {
        format!(
            "first_name: {}\nlast_name: {}\nage: {}\n",
            self.first_name, self.last_name, self.age
        )
    }
    pub fn introduce(&self) -> String {
        format!(
            "Hi, I'm {} and I'm {} years old\n",
            self.full_name(),
            self.age
        )
    }
}
