use core::marker::PhantomData;

/***** TRAITS *****/

pub trait InOut {
    unsafe fn port_in(port: u16) -> Self;
    unsafe fn port_out(port: u16, value: Self);
}

impl InOut for u8 {
    unsafe fn port_in(port: u16) -> u8 { inb(port) }
    unsafe fn port_out(port: u16, value: u8) { outb(port, value); }
}

// impl InOut for u16 {
//     unsafe fn port_in(port: u16) -> u16 { inw(port) }
//     unsafe fn port_out(port: u16, value: u16) { outw(port, value); }
// }
//
// impl InOut for u32 {
//     unsafe fn port_in(port: u16) -> u32 { inl(port) }
//     unsafe fn port_out(port: u16, value: u32) { outl(port, value); }
// }

/***** STRUCTS & ENUMS *****/

pub struct Port<T: InOut> {
    port: u16,
    phantom: PhantomData<T>
}

impl<T: InOut> Port<T> {
    pub const unsafe fn new(port: u16) -> Port<T> {
        Port { port: port, phantom: PhantomData }
    }

    pub fn read(&mut self) -> T {
        unsafe { T::port_in(self.port) }
    }

    pub fn write(&mut self, value: T) {
        unsafe { T::port_out(self.port, value); }
    }
}

pub struct PortPair<T: InOut> {
    control: Port<T>,
    data: Port<T>
}

impl<T: InOut> PortPair<T> {
    pub const unsafe fn new(control: u16, data: u16) -> PortPair<T> {
        PortPair { control: Port::new(control), data: Port::new(data) }
    }

    pub fn write(&mut self, control: T, value: T) {
        self.control.write(control);
        self.data.write(value);
    }

    pub fn read(&mut self, control: T) -> T {
        self.control.write(control);
        self.data.read()
    }
}

/***** FUNCTIONS *****/

unsafe fn inb(port: u16) -> u8 {
    let ret: u8;
    asm!("inb $1, $0" : "={ax}"(ret) : "N{dx}"(port) : : "volatile");
    ret
}

unsafe fn outb(port: u16, value: u8) {
    asm!("outb $0, $1" : : "{ax}"(value), "N{dx}"(port) : : "volatile");
}
