use std::fmt::{Display, Formatter, Error};

const MEM_SIZE: usize = 4096 / 8;

pub struct Memory {
    memory: [u8; MEM_SIZE]
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            memory: [0; MEM_SIZE]
        }
    }
}

impl Display for Memory {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        for (i, value) in self.memory.iter().enumerate() {
            write!(f, "0x{:X} ", value)?;
            if (i + 1) % 15 == 0 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}
