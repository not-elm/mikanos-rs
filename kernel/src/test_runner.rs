use kernel_lib::gop::console::init_console;
use kernel_lib::println;

pub trait Testable {
    fn run(&self) -> ();
}


impl<T> Testable for T
    where
        T: Fn(),
{
    fn run(&self) {
        println!("test name={}", core::any::type_name::<T>());
        self();
        println!("[ok]");
    }
}

#[cfg(test)]
pub fn my_runner(tests: &[&dyn Testable]) {
    println!("start test! num={}", tests.len());
    for t in tests {
        t.run();
    }
}

#[test_case]
fn it_should() {
    assert_eq!(0, 0);
}

