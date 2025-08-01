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

            // TODO: decide on what to do, panic or Unknown no-ack: _ => panic!("invalid enum discriminant"),
        }
    }

    pub fn lower(&self) -> u8 {
        match self {
            NoAcknowledgeSource::Address => 0,
            NoAcknowledgeSource::Data => 1,
            _ => 2,

            // TODO: decide on what to do, panic or Unknown no-ack: _ => panic!("invalid enum discriminant"),
        }
    }
}

#[derive(Debug)]
pub enum ErrorCode {
    /// No error occurred. Operation was succesful.
    None,
    /// Bus error occurred. e.g. A START or a STOP condition is detected and
    /// is not located after a multiple of 9 SCL clock pulses.
    Bus,
    /// The arbitration was lost, e.g. electrical problems with the clock signal.
    ArbitrationLoss,
    /// A bus operation was not acknowledged, e.g. due to the addressed
    /// device not being available on the bus or the device not being ready
    /// to process requests at the moment.
    NoAcknowledge(NoAcknowledgeSource),
    /// The peripheral receive buffer was overrun.
    Overrun,
    /// A different error occurred.
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

impl From<ErrorCode> for u8 {
    fn from(value: ErrorCode) -> Self {
        ErrorCode::lower(&value)
    }
}

/*
impl From<u8> for ErrorCode {
    fn from(value: u8) -> Self {
        ErrorCode::lift(value)
    }
}

impl From<u8> for NoAcknowledgeSource {
    fn from(value: u8) -> Self {
        NoAcknowledgeSource::lift(value)
    }
}

impl Into<u8> for ErrorCode {
    fn into(self) -> u8 {
        self.lower()
    }
}

impl Into<u8> for NoAcknowledgeSource {
    fn into(self) -> u8 {
        self.lower()
    }
}
*/
