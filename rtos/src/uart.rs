// FE310
pub const UART0_BASE_ADDR: usize = 0x10013000;
pub const UART1_BASE_ADDR: usize = 0x10023000;

pub const GPIO_BLOCK0_BASE_ADDR: usize = 0x20023000;

pub struct Uart {
    base_address: usize,
}

impl Uart {
    pub fn from_base_address(base_address: usize) -> Self {
        Uart { base_address }
    }

    pub fn init(&self, baund_rate: u32) {
        unsafe {
            if baund_rate != 115200 {
                panic!("Invalid baund")
            }

            let gpio_iof_register = (GPIO_BLOCK0_BASE_ADDR + 0x38) as *mut u32;

            // Enable IOF functionality for uart0 rx gpio pin 16
            gpio_iof_register.write_volatile(gpio_iof_register.read_volatile() | (1 << 16));

            // Enable IOF functionality for uart0 tx gpio pin17
            gpio_iof_register.write_volatile(gpio_iof_register.read_volatile() | (1 << 17));

            let div_register = (self.base_address + 0x18) as *mut u32;
            div_register.write_volatile(138); // baund 115200 at tlclk 16Mhz

            // At reset/default, number of stop bits in txctl is 1
            // No tx/rx watermark interrupt

            // Enable tx
            let txctl = (self.base_address + 0x8) as *mut u32;
            txctl.write_volatile(txctl.read_volatile() | 0x1 as u32);

            // Enable rx
            let rx_ctrl = (self.base_address + 0xC) as *mut u32;
            rx_ctrl.write_volatile(rx_ctrl.read_volatile() | 0x1 as u32);
        }
    }

    //pub fn send_byte(&self) {}

    //
    //-------------------------
    //|FULL | reserved | data |
    //-------------------------
    //|31   | 30:8     |[7:0] |
    //-------------------------
    //
    pub fn send_byte_blocking(&self, byte: u8) {
        unsafe {
            // Write to UART data register
            let tx_register_data = self.base_address as *mut u8;
            let tx_register_full = self.base_address as *mut u32;
            let mut t = tx_register_full.read_volatile();

            while (t & 0x8000_0000 as u32) != 0 {
                t = tx_register_full.read_volatile();
            }

            tx_register_data.write_volatile(byte);
        }
    }

    //
    //-------------------------
    //|FULL | reserved | data |
    //-------------------------
    //|31   | 30:8     |[7:0] |
    //-------------------------
    //
    pub fn receive_byte(&self) -> u8 {
        unsafe {
            // Read from UART data register

            let data_register = (self.base_address + 0x04) as *mut u8;
            data_register.read_volatile()
        }
    }
}
