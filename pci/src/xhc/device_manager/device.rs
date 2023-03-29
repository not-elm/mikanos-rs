use xhci::context::{DeviceHandler, Input64Byte, InputHandler};

#[derive(Debug)]
pub struct Device {
    slot_id: u8,
    input_context: Input64Byte,
    device_context: xhci::context::Device64Byte,
}

impl Device {
    pub fn new_with_init_slot_context(parent_hub_slot_id: u8, port_speed: u8, slot_id: u8) -> Self {
        let mut me = Self::new(slot_id);
        let slot = me.input_context.device_mut().slot_mut();
        slot.set_parent_hub_slot_id(parent_hub_slot_id);
        slot.set_route_string(0);
        slot.set_context_entries(1);
        slot.set_speed(port_speed);
        me
    }

    fn init_end_point_context0(&mut self) {
        self.device_context.endpoint_mut(1).set_tr_dequeue_pointer()
    }
    fn new(slot_id: u8) -> Self {
        Self {
            slot_id,
            input_context: Input64Byte::new_64byte(),
        }
    }

    pub fn slot_id(&self) -> u8 {
        self.slot_id
    }
}
