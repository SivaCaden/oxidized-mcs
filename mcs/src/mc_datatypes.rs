



pub struct McBool {
    cond: bool
}

impl McBool {
    pub fn new(value: i8) -> McBool{
        match value {
            0x01 => {
                McBool{cond:true}
            }
            0x00 => {
                McBool{cond:false}
            }
            _ => {panic!("not expected 0x01 or 0x00, got : {value}")}
        }
    }
    pub fn val(self) -> bool {
        self.cond
    }
    pub fn to_mc(self) -> i8 {
         match self.cond {
             true => { 0x01 }
             false => { 0x00 }
         }
    }
}

pub struct McByte {
    value: i8,
}

impl McByte {
    pub fn new(val: i8) -> McByte {
        McByte{ value:val }
    }
    pub fn val(self) -> i8 {
       self.value 
    }
}

pub struct McUByte { value: u8 }

impl McUByte {
    pub fn new(val: u8) -> McUByte { McUByte{value: val}}

    pub fn val(self) -> u8 { self.value }

}
