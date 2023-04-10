use crate::configuration_space::io::config_address_register::ConfigAddrRegister;

pub mod mock_memory_accessor;
pub mod real_memory_accessor;


const CONFIG_DATA_REGISTER_ADDR: u16 = 0x0CFC;


/// このトレイトはI/O空間への入出力を提供します。
pub trait IoMemoryAccessible {
    /// 指定のポートアドレスから値を読み込みます。
    fn io_in(&self, port: u16) -> u32;


    /// 指定のポートアドレスに値を書き込みます。
    fn io_out(&mut self, port: u16, value: u32);


    /// Config Address Registerへの値の書き込みと、
    /// Config Data Registerから値の読み込みを行います。
    fn read_config_data_with_set_addr(&mut self, config_addr_register: ConfigAddrRegister) -> u32 {
        self.write_config_addr(config_addr_register);
        self.read_config_data()
    }

    /// Config Address Registerへの値の書き込みと、
    /// Config Data Registerへの値の書き込みを行います。
    fn write_config_data_with_set_addr(
        &mut self,
        config_addr_register: ConfigAddrRegister,
        value: u32,
    ) {
        self.write_config_addr(config_addr_register);
        self.write_config_data(value);
    }


    fn write_config_addr(&mut self, config_addr_register: ConfigAddrRegister) {
        const CONFIG_ADDR_REGISTER_ADDR: u16 = 0x0CF8;

        self.io_out(CONFIG_ADDR_REGISTER_ADDR, config_addr_register.as_data())
    }


    fn write_config_data(&mut self, value: u32) {
        self.io_out(CONFIG_DATA_REGISTER_ADDR, value);
    }


    fn read_config_data(&self) -> u32 {
        self.io_in(CONFIG_DATA_REGISTER_ADDR)
    }
}
