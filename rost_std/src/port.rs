pub trait PortMessage {
    unsafe fn send(self, address: u16);
    unsafe fn receive(address: u16) -> Self;
}

impl PortMessage for u8 {
    unsafe fn send(self, address: u16) {
        asm!("outb %al, %dx" :: "{dx}"(address), "{al}"(self) :: "volatile");
    }
    
    #[inline]
    unsafe fn receive(address: u16) -> u8 {
        let value: u8;
        asm!("inb %dx, %al" : "={al}"(value) : "{dx}"(address) :: "volatile");
        value
    }
}

impl PortMessage for u16 {
    unsafe fn send(self, address: u16) {
        asm!("outw %ax, %dx" :: "{dx}"(address), "{al}"(self) :: "volatile");
    }
    
    #[inline]
    unsafe fn receive(address: u16) -> u16 {
        let value: u16;
        asm!("inw %dx, %ax" : "={al}"(value) : "{dx}"(address) :: "volatile");
        value
    }
}

impl PortMessage for u32 {
    unsafe fn send(self, address: u16) {
        asm!("outl %eax, %dx" :: "{dx}"(address), "{al}"(self) :: "volatile");
    }
    
    #[inline]
    unsafe fn receive(address: u16) -> u32 {
        let value: u32;
        asm!("inl %dx, %eax" : "={al}"(value) : "{dx}"(address) :: "volatile");
        value
    }
}

pub unsafe fn read<T: PortMessage>(address: u16) -> T {
    T::receive(address)
} 

pub unsafe fn write<T: PortMessage>(address: u16, value: T) {
    T::send(value, address);
}
