///
/// ## Document
///
/// [P406](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf)
pub trait PortStatusAndControlAccessible {
    /// 1 = A device is connected
    fn current_connect(&self) -> bool;
}
