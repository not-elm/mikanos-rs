use crate::gop::console::console_builder::ConsoleBuilder;
use crate::gop::console::console_writer::ConsoleWriter;
use common_lib::frame_buffer::FrameBufferConfig;

pub mod console_builder;
pub mod console_writer;

pub struct GlobalConsole(Option<ConsoleWriter>);
pub static mut CONSOLE: GlobalConsole = GlobalConsole(None);

impl GlobalConsole {
    pub fn init(&mut self, frame_buffer_config: FrameBufferConfig) {
        let console = ConsoleBuilder::new().build(frame_buffer_config);
        self.0 = Some(console);
    }

    pub fn get_mut(&mut self) -> &mut ConsoleWriter {
        self.0.as_mut().unwrap()
    }
}

pub fn init_console(frame_buffer_config: FrameBufferConfig) {
    unsafe { CONSOLE.init(frame_buffer_config) };
}

pub fn get_mut_console() -> &'static mut ConsoleWriter {
    unsafe { CONSOLE.get_mut() }
}
