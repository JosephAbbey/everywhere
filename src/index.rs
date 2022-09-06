everywhere!(
    pub fn hi(name: String) -> String {
        "Hi ".to_string() + &name + "!"
    }

    pub fn hello(fname: String, lname: String) -> String {
        "Hello ".to_string() + &fname + " " + &lname + "."
    }

    pub fn add(a: f64, b: f64) -> f64 {
        a + b
    }
);
