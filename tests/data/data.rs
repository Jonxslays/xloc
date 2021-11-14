struct TestStruct<'a> {
    message: &'a str
}

impl<'a> TestStruct<'a> {
    fn new(message: &'a str) -> Self {
        Self { message }
    }
}

fn main() {
    let t = TestStruct::new("Fearless-concurrency! Resistance: futile");
    let c = 'c';
    println!("{}", t.message);
    println!("{}", c);
}
