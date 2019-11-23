use std::cell::RefCell;
use crate::bus::Bus;

struct Ppu {
    bus: RefCell<Bus>,
    // Registers
    controller:  u8, // 0x2000
    mask:        u8, // 0x2001
    status:      u8, // 0x2002
    oam_address: u8, // 0x2003
    oam_data:    u8, // 0x2004
    scroll:      u8, // 0x2005
    address:     u8, // 0x2006
    data:        u8, // 0x2007
}

impl Ppu {
    pub fn new(bus: RefCell<Bus>) -> Ppu {
        Ppu {
            bus,
            controller:  0,
            mask:        0,
            status:      0,
            oam_address: 0,
            oam_data:    0,
            scroll:      0,
            address:     0,
            data:        0,
        }
    }

    fn update_registers(&mut self) {
        self.controller = self.bus.borrow().read_cpu_ram(0x2000);
        self.mask = self.bus.borrow().read_cpu_ram(0x2001);
        self.status = self.bus.borrow().read_cpu_ram(0x2002);
        self.oam_address = self.bus.borrow().read_cpu_ram(0x2003);
        self.oam_data = self.bus.borrow().read_cpu_ram(0x2004);
        self.scroll = self.bus.borrow().read_cpu_ram(0x2005);
        self.address = self.bus.borrow().read_cpu_ram(0x2006);
        self.data = self.bus.borrow().read_cpu_ram(0x2007);
    }
}
