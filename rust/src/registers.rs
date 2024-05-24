use crate::proto;

/// Memory registers for keeping execution state
#[derive(Debug, Copy, Clone)]
pub enum Register {
    /// (Status Code) Current execution status code. 0 for success
    SC,
    /// (Status Flags) Contains a bitmask representing all of the state flags
    SF,
    /// (Program Counter) Keeps the current instruction\
    /// _Note:_ This register is disabled on TX mode
    PC,
    /// (Previous Program Counter) Keeps a copy the previous program counter when branching
    PP,
    /// (Return Value), Contains the returned value by the last routine that called RET
    RV,
    /// (Inclusive Range Start) Absolute index of the currently selected start position
    R0,
    /// (Exclusive Range End) Absolute index of the currently selected end position
    R1,
    /// Current amount of pending operations
    PO,
    /// General purpose registers from $0-$31
    GeneralPurpose(u8),
}

impl Into<u8> for Register {
    /// Get the absolute memory address of the register from the protobuf definition\
    /// General Purpose Registers have an offset, which is defined as GENERAL
    fn into(self) -> u8 {
        match self {
            Register::SC => proto::Registers::Sc as u8,
            Register::SF => proto::Registers::Sf as u8,
            Register::PC => proto::Registers::Pc as u8,
            Register::PP => proto::Registers::Pp as u8,
            Register::RV => proto::Registers::Rv as u8,
            Register::R0 => proto::Registers::R0 as u8,
            Register::R1 => proto::Registers::R1 as u8,
            Register::PO => proto::Registers::Po as u8,
            Register::GeneralPurpose(n) => (proto::Registers::General as u8) + n,
        }
    }
}
