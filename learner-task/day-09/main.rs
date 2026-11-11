trait HasCode {
    fn code(&self) -> &str;
}

struct Observation {
    code: String,
}

struct ObservationTemplate {
    code: &'static str,
}

impl HasCode for Observation {
    fn code(&self) -> &str {
        &self.code
    }
}

impl HasCode for ObservationTemplate {
    fn code(&self) -> &str {
        self.code
    }
}

// The trait bound lets this function work with any type that provides a code.
fn code_of<T: HasCode>(item: &T) -> &str {
    item.code()
}

fn print_code(item: &dyn HasCode) {
    println!("Dynamic code: {}", item.code());
}

fn main() {
    let observation = Observation {
        code: String::from("HR"),
    };
    let template = ObservationTemplate { code: "TEMP" };

    println!("Generic code: {}", code_of(&observation));
    println!("Generic code: {}", code_of(&template));
    print_code(&observation);
}
