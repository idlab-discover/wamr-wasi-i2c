pub type I2cAddress = u16;
pub type I2cResourceHandle = u32;

#[repr(u8)]
#[derive(Debug)]
pub enum NoAcknowledgeSource {
    Address,
    Data,
    Unknown,
}

impl NoAcknowledgeSource {
    pub fn lift(val: u8) -> NoAcknowledgeSource {
        match val {
            0 => NoAcknowledgeSource::Address,
            1 => NoAcknowledgeSource::Data,
            _ => NoAcknowledgeSource::Unknown,
        }
    }

    pub fn lower(&self) -> u8 {
        match self {
            NoAcknowledgeSource::Address => 0,
            NoAcknowledgeSource::Data => 1,
            _ => 2,
        }
    }
}

#[derive(Debug)]
pub enum ErrorCode {
    None,
    Bus,
    ArbitrationLoss,
    NoAcknowledge(NoAcknowledgeSource),
    Overrun,
    Other,
}

impl ErrorCode {
    pub fn lower(&self) -> u8 {
        match self {
            ErrorCode::None => 0b000_00000,
            ErrorCode::Bus => 0b001_00000,
            ErrorCode::ArbitrationLoss => 0b010_00000,
            ErrorCode::NoAcknowledge(source) => {
                let no_ack_bits = source.lower();
                0b011_00000 | no_ack_bits
            }
            ErrorCode::Overrun => 0b100_00000,
            _ => 0b101_00000,
        }
    }

    pub fn lift(val: u8) -> ErrorCode {
        let error_type = val >> 5; // take first 3 bits only
        let error_variant = 0b000_11111 & val; // take last 5 bits only
        match error_type {
            0 => ErrorCode::None,
            1 => ErrorCode::Bus,
            2 => ErrorCode::ArbitrationLoss,
            3 => ErrorCode::NoAcknowledge(NoAcknowledgeSource::lift(error_variant)),
            4 => ErrorCode::Overrun,
            _ => ErrorCode::Other,
        }
    }
}
