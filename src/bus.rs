pub struct Bus {
    data: [u8; 0x1_0000],
}
impl Bus {
    pub fn new() -> Bus {
        Bus {
            data: [0x00; 0x1_0000],
        }
    }
    pub fn read_bytes(&self, address: u16) -> u8 {
        let add = address as usize;
        return self.data[add];
    }
    pub fn read_bytes_range(&self, address: u16, length: u16) -> &[u8] {
        let add = address as usize;
        let len = length as usize;
        let add_end = add + len;
        if add_end > len {
            panic!("Trying to read out of bus memory");
        }
        return &self.data[add..add_end];
    }
    pub fn write_bytes(&mut self, address: u16, value: u8) {
        self.data[address as usize] = value;
    }
    pub fn write_slice(&mut self, address: u16, slice: &[u8]) {
        let add = address as usize;
        let add_end = add + slice.len();
        if add_end > self.data.len() {
            panic!("Trying to write out of bus memory");
        }
        let data_slice = &mut self.data[add..add_end];
        data_slice.copy_from_slice(slice);
    }
}

#[cfg(test)]
mod tests {
    use crate::bus::Bus;
    #[test]
    fn test_read() {
        let mut bus = Bus {
            data: [0x00; 0x1_0000],
        };
        bus.data[0x0003] = 0xFF;
        bus.data[0xF003] = 0xFC;
        bus.data[0xFFFF] = 0xFC;
        assert_eq!(bus.read_bytes(0x0003), 0xFF);
        assert_eq!(bus.read_bytes(0x0001), 0x00);
        assert_eq!(bus.read_bytes(0xF003), 0xFC);
        assert_eq!(bus.read_bytes(0xFFFF), 0xFC);

        assert_ne!(bus.read_bytes(0x0000), 0xA0);
    }
    #[test]
    fn test_write() {
        let mut bus = Bus {
            data: [0x00; 0x1_0000],
        };
        assert_ne!(bus.read_bytes(0x0010), 0x12);
        bus.write_bytes(0x0010, 0x12);
        assert_eq!(bus.read_bytes(0x0010), 0x12);
    }
    #[test]
    fn read_slice() {
        let mut bus = Bus {
            data: [0x00; 0x1_0000],
        };
        bus.write_slice(0x00F0, &[1,2,3,4,5]);
        assert_eq!(bus.data[0x00F0],1);
        assert_eq!(bus.data[0x00F1],2);
        assert_eq!(bus.data[0x00F2],3);
    }
    #[test]
    fn write_slice() {
        let mut bus = Bus {
            data: [0x00; 0x1_0000],
        };
        bus.write_slice(0x0000, &[1, 2, 3, 4]);
        assert_eq!(bus.data[0x0002], 3);
        bus.write_slice(0xFFFF, &[4]);
        assert_eq!(bus.data[0xFFFF], 4);
    }
    #[test]
    #[should_panic(expected = "Trying to read out of bus memory")]
    fn read_slice_panic() {
        let bus = Bus {
            data: [0x00; 0x1_0000],
        };
        bus.read_bytes_range(0xFFFF, 2);
    }
    #[test]
    #[should_panic(expected = "Trying to write out of bus memory")]
    fn write_slice_panic() {
        let mut bus = Bus {
            data: [0x00; 0x1_0000],
        };
        bus.write_slice(0xFFFF, &[1, 2]);
    }
}
