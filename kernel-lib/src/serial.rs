use lazy_static::lazy_static;
use uart_16550::SerialPort;

lazy_static! {
    pub static ref SERIAL: spin::Mutex<SerialPort> = new_serial_port();
}

/// シリアルポートを扱うための構造体を返します。
///
/// 詳細は[Writing OS in Rust](https://os.phil-opp.com/ja/testing/)
fn new_serial_port() -> spin::Mutex<SerialPort> {
    let mut serial_port = unsafe { SerialPort::new(0x03_F8) };
    serial_port.init();

    spin::Mutex::new(serial_port)
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;

    SERIAL
        .lock()
        .write_fmt(args)
        .expect("Printing to serial failed");
}


#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}


#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}
