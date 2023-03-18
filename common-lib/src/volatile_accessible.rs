pub trait VolatileAccessible<ActualValue, Addr> {
    fn new_uncheck(v: Addr) -> Self;
    fn read_volatile(&self) -> ActualValue;
    fn read_flag_volatile(&self) -> bool;

    fn write_flag_volatile(&self, flag: bool);
    fn write_volatile(&self, value: ActualValue);
}
