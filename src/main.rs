#[macro_use]
extern crate qmlrs;

struct Factorial;
impl Factorial {
    fn calculate(&self, x: i64) -> i64 {
        (1 ..= x).fold(1, |t,c| t * c)
    }
}

Q_OBJECT! { Factorial:
    slot fn calculate(i64);
}

fn main() {
    let mut engine = qmlrs::Engine::new();

    engine.set_property("factorial", Factorial);
    engine.load_local_file("src/neatoui.qml");

    engine.exec();
}
