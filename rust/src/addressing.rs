use crate::registers::Register;

/// Use an immediate value or use one stored in memory for values
#[derive(Debug, Copy, Clone)]
pub enum AddressingMode {
    /// Use the explicitly provided value
    Immediate(u8),
    /// Use the value stored in the register at this memory address
    Indirect(Register),
}