use crate::proto;

/// Designates the code for a given effect
#[derive(Debug, Copy, Clone)]
pub struct EffectCode(pub u8);

/// Designates the code for a given delay time unit
#[derive(Debug, Copy, Clone)]
pub enum DelayCode {
    /// Delay for milliseconds
    MS,
    /// Delay for seconds
    SEC,
    /// Delay for minutes
    MIN,
    /// Delay for hours
    HRS,
}

impl Into<proto::DelayCode> for DelayCode {
    /// Transform a [DelayCode] into its proto definition
    fn into(self) -> proto::DelayCode {
        match self {
            DelayCode::MS => proto::DelayCode::Ms,
            DelayCode::SEC => proto::DelayCode::Sec,
            DelayCode::MIN => proto::DelayCode::Min,
            DelayCode::HRS => proto::DelayCode::Hrs,
        }
    }
}

impl Into<u8> for DelayCode {
    /// Get a [DelayCode] binary representation based on proto definition
    fn into(self) -> u8 {
        let delay_code: proto::DelayCode = self.into();
        delay_code as u8
    }
}