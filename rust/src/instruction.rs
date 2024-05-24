use crate::addressing::AddressingMode;
use crate::arrays::{Array2, Array3};
use crate::codes::{DelayCode, EffectCode};
use crate::proto;
use crate::registers::Register;

/// Set of instructions available to Prism Assembly Language and Prism Binary Format
#[derive(Debug, Copy, Clone)]
pub enum InstructionSet {
    /// (No Operation) Do nothing
    NOP,
    /// Marks the beginning of a new script (Clears the machine state)\
    /// marks TRANSMIT flag as False, SCRIPT flag as True and HALT flag as False\
    /// must always be the first instruction inside a script, the script starts running when RUN is reached
    BEGIN,
    /// Marks the end of a script (important!) and starts running it
    RUN,
    /// Disables the state machine (no branching, no $PC) for transmitting real time data\
    /// marks TRANSMIT flag as True, SCRIPT flag as False and HALT flag as False\
    /// TRANSMIT mode is the default mode at startup\
    /// Beware: TRANSMIT mode disables branching as there is no state to keep
    TRANSMIT,
    /// Stop execution, sets the HALT flag to True and set the $SC with the value provided
    HALT(AddressingMode),
    /// (Absolute Indexing) Use absolute indexing when the size of the buffer is unknown\
    /// marks IX flag as True\
    /// Beware: Absolute index can cause buffer overflowing, in that case the OW flag is set
    AIDX,
    /// (Relative InDeXing) Use relative indexing mapping from (0 - 100%) of the total buffer size\
    /// marks IX flag as False
    RIDX,
    /// Hold instructions until an UPDATE is issued\
    /// marks HOLD flag as True
    HOLD,
    /// (No) hold, makes instructions effects immediate, effectively disabling UPDATE\
    /// if NHOLD is called before UPDATE then the instructions are dropped\
    /// marks HOLD flag as False and resets $PO
    NHOLD,
    /// Apply instructions on hold\
    /// resets $PO (pending operations)
    UPDATE,
    /// (Jump) to another part of the script\
    /// copies $PC value into $PP and then sets $PC to the provided value\
    /// Beware: This instructions (and every other branching instruction) is disabled on TRANSMIT mode
    JMP(u8),
    /// (Return) from a JMP\
    /// copies $PP into $PC, and sets the value of $RV (return value)\
    /// Beware: This instructions (and every other branching instruction) is disabled on TRANSMIT mode
    RET(AddressingMode),
    /// (Branch if Equal) Does a JMP only if arguments A and B are equal\
    /// Beware: This instructions (and every other branching instruction) is disabled on TRANSMIT mode\
    /// no addressing mode supported on third (C) parameter, it must reference a label or absolute position in the script
    BEQ(AddressingMode, AddressingMode, u8),
    /// (Branch if Not Equal) Does a JMP only if arguments A and B are NOT equal\
    /// Beware: This instructions (and every other branching instruction) is disabled on TRANSMIT mode\
    /// no addressing mode supported on third (C) parameter, it must reference a label or absolute position in the script
    BNE(AddressingMode, AddressingMode, u8),
    /// Load a new value into a register
    LOAD(Register, AddressingMode),
    /// Add a given value to a register's contents
    ADD(Register, AddressingMode),
    /// (Substract) a given value to a register's contents
    SUB(Register, AddressingMode),
    /// Change the color of a range of LEDs specifiyng all of the three HSL parameters
    FILL(Array2<AddressingMode>, Array3<AddressingMode>),
    /// Change the color of a range of LEDs specifying only Hue (HSL)
    HFILL(Array2<AddressingMode>, AddressingMode),
    /// Change the color of a range of LEDs specifying only Saturation (HSL)
    SFILL(Array2<AddressingMode>, AddressingMode),
    /// Change the color of a range of LEDs specifying only Level (HSL)
    LFILL(Array2<AddressingMode>, AddressingMode),
    /// Change the color of only one LED specifiyng all of the three HSL parameters
    PAINT(AddressingMode, Array3<AddressingMode>),
    /// Change the color of only one LED specifiyng only Hue (HSL)
    HPAINT(AddressingMode, AddressingMode),
    /// Change the color of only one LED specifiyng only Saturation (HSL)
    SPAINT(AddressingMode, AddressingMode),
    /// Change the color of only one LED specifiyng only Level (HSL)
    LPAINT(AddressingMode, AddressingMode),
    /// Apply an effect to a range of LEDs
    /// First argument ([EffectCode]) is not considered a parameter (A or B)
    EFFECT(EffectCode, Array2<AddressingMode>, AddressingMode),
    /// Delay execution for a given amount of time\
    /// First argument ([DelayCode]) is not considered a parameter (A or B)
    DELAY(DelayCode, AddressingMode),
    /// Pause the script execution, can be resumed with RUN
    PAUSE,
    /// Get the contents of a registers\
    /// some devices might not be able to transmit data
    GET(Register),
    /// Does a complete restart of all registers, flags, and memory contents
    RESET,
}

/// Select either the first (A) or second (B) parameter
enum ParameterType {
    A,
    B,
}

/// Return the parsed address and alter the given [mask] to the correct addressing mode for the parameter specified in [parameter_type]
fn match_addressing(addrm: AddressingMode, mask: &mut u8, parameter_type: ParameterType) -> u8 {
    match addrm {
        AddressingMode::Immediate(addr) => {
            // Set addressing to immediate
            *mask &= match parameter_type {
                ParameterType::A => !(proto::AddressingMode::AIndirect as u8),
                ParameterType::B => !(proto::AddressingMode::BIndirect as u8),
            };
            // Return the byte
            addr
        }
        AddressingMode::Indirect(register) => {
            // Set addressing to indirect
            *mask |= match parameter_type {
                ParameterType::A => proto::AddressingMode::AIndirect as u8,
                ParameterType::B => proto::AddressingMode::BIndirect as u8,
            };
            // Return the register address
            register.into()
        }
    }
}

/// Transform the instruction into binary format
impl Into<Vec<u8>> for InstructionSet {
    /// Assemble the current instructino into Prism Binary Format
    fn into(self) -> Vec<u8> {
        // Where to store the bytes
        let mut result: Vec<u8> = vec![];

        // Get the instruction protobuf
        let instruction: proto::InstructionSet = self.into();

        // First instruction byte (with room for addressing type)
        let mut instruction_byte = (instruction as u8) << 2;

        match self {
            // These instructions are 1 byte only
            InstructionSet::NOP
            | InstructionSet::BEGIN
            | InstructionSet::RUN
            | InstructionSet::TRANSMIT
            | InstructionSet::AIDX
            | InstructionSet::RIDX
            | InstructionSet::HOLD
            | InstructionSet::NHOLD
            | InstructionSet::UPDATE
            | InstructionSet::PAUSE
            | InstructionSet::RESET => {
                // Insert the instruction byte and nothing more
                result.push(instruction_byte);
            }

            InstructionSet::RET(addrm) | InstructionSet::HALT(addrm) => {
                // Get the addressing mode and alter the addressing mask on the instruction
                let next_byte = match_addressing(addrm, &mut instruction_byte, ParameterType::A);
                // Insert data
                result.push(instruction_byte);
                result.push(next_byte);
            }

            InstructionSet::JMP(label) => {
                // Insert instruction
                result.push(instruction_byte);
                result.push(label);
            }

            InstructionSet::BEQ(a, b, label) | InstructionSet::BNE(a, b, label) => {
                // Get the addressing mode and alter the addressing mask on the instruction
                let a_byte = match_addressing(a, &mut instruction_byte, ParameterType::A);
                let b_byte = match_addressing(b, &mut instruction_byte, ParameterType::B);

                // Insert instruction
                result.push(instruction_byte);
                result.push(a_byte);
                result.push(b_byte);
                result.push(label);
            }

            InstructionSet::LOAD(register, value)
            | InstructionSet::ADD(register, value)
            | InstructionSet::SUB(register, value) => {
                // Get the addressing mode and alter the addressing mask on the instruction
                let a_byte = register.into();
                let b_byte = match_addressing(value, &mut instruction_byte, ParameterType::B);

                // Insert instruction
                result.push(instruction_byte);
                result.push(a_byte);
                result.push(b_byte);
            }

            InstructionSet::FILL(range, paint) => {
                // Get the addressing mode and alter the addressing mask on the instruction
                let array: Vec<AddressingMode> = range.into();
                let mut a_bytes: Vec<u8> = array
                    .iter()
                    .map(|&x| match_addressing(x, &mut instruction_byte, ParameterType::A))
                    .collect();

                let array: Vec<AddressingMode> = paint.into();
                let mut b_bytes: Vec<u8> = array
                    .iter()
                    .map(|&x| match_addressing(x, &mut instruction_byte, ParameterType::B))
                    .collect();

                // Insert instruction
                result.push(instruction_byte);
                result.append(&mut a_bytes);
                result.append(&mut b_bytes);
            }

            InstructionSet::HFILL(range, paint)
            | InstructionSet::SFILL(range, paint)
            | InstructionSet::LFILL(range, paint) => {
                // Get the addressing mode and alter the addressing mask on the instruction
                let array: Vec<AddressingMode> = range.into();
                let mut a_bytes: Vec<u8> = array
                    .iter()
                    .map(|&x| match_addressing(x, &mut instruction_byte, ParameterType::A))
                    .collect();

                let b_byte = match_addressing(paint, &mut instruction_byte, ParameterType::B);

                // Insert instruction
                result.push(instruction_byte);
                result.append(&mut a_bytes);
                result.push(b_byte);
            }

            InstructionSet::PAINT(addr, paint) => {
                // Get the addressing mode and alter the addressing mask on the instruction
                let a_byte = match_addressing(addr, &mut instruction_byte, ParameterType::A);

                // Get the array as bytes
                let array: Vec<AddressingMode> = paint.into();
                let mut b_bytes: Vec<u8> = array
                    .iter()
                    .map(|&x| match_addressing(x, &mut instruction_byte, ParameterType::B))
                    .collect();

                // Insert instruction
                result.push(instruction_byte);
                result.push(a_byte);
                result.append(&mut b_bytes);
            }

            InstructionSet::HPAINT(addr, paint)
            | InstructionSet::SPAINT(addr, paint)
            | InstructionSet::LPAINT(addr, paint) => {
                // Get the addressing mode and alter the addressing mask on the instruction
                let a_byte = match_addressing(addr, &mut instruction_byte, ParameterType::A);
                let b_byte = match_addressing(paint, &mut instruction_byte, ParameterType::B);

                // Insert instruction
                result.push(instruction_byte);
                result.push(a_byte);
                result.push(b_byte);
            }

            InstructionSet::EFFECT(code, range, value) => {
                // Get the addressing mode and alter the addressing mask on the instruction
                let array: Vec<AddressingMode> = range.into();
                let mut a_bytes: Vec<u8> = array
                    .iter()
                    .map(|&x| match_addressing(x, &mut instruction_byte, ParameterType::A))
                    .collect();

                let b_byte = match_addressing(value, &mut instruction_byte, ParameterType::B);

                // Insert instruction
                result.push(instruction_byte);
                // Insert the effect code
                result.push(code.0);
                result.append(&mut a_bytes);
                result.push(b_byte);
            }

            InstructionSet::DELAY(code, addrm) => {
                // Get the addressing mode and alter the addressing mask on the instruction
                let next_byte = match_addressing(addrm, &mut instruction_byte, ParameterType::A);
                // Insert instruction
                result.push(instruction_byte);
                // Insert delay code
                result.push(code as u8);
                // Insert delay amount
                result.push(next_byte);
            }

            InstructionSet::GET(register) => {
                // Insert data
                result.push(instruction_byte);
                result.push(register.into());
            }
        }

        // Return bytes
        result
    }
}

impl Into<proto::InstructionSet> for InstructionSet {
    /// Transform [InstructionSet] into protobuf definition [proto::InstructionSet]
    fn into(self) -> proto::InstructionSet {
        match self {
            InstructionSet::NOP => proto::InstructionSet::Nop,
            InstructionSet::BEGIN => proto::InstructionSet::Begin,
            InstructionSet::RUN => proto::InstructionSet::Run,
            InstructionSet::TRANSMIT => proto::InstructionSet::Transmit,
            InstructionSet::HALT(_) => proto::InstructionSet::Halt,
            InstructionSet::AIDX => proto::InstructionSet::Aidx,
            InstructionSet::RIDX => proto::InstructionSet::Ridx,
            InstructionSet::HOLD => proto::InstructionSet::Hold,
            InstructionSet::NHOLD => proto::InstructionSet::Nhold,
            InstructionSet::UPDATE => proto::InstructionSet::Update,
            InstructionSet::JMP(_) => proto::InstructionSet::Jmp,
            InstructionSet::RET(_) => proto::InstructionSet::Ret,
            InstructionSet::BEQ(_, _, _) => proto::InstructionSet::Beq,
            InstructionSet::BNE(_, _, _) => proto::InstructionSet::Bne,
            InstructionSet::LOAD(_, _) => proto::InstructionSet::Load,
            InstructionSet::ADD(_, _) => proto::InstructionSet::Add,
            InstructionSet::SUB(_, _) => proto::InstructionSet::Sub,
            InstructionSet::FILL(_, _) => proto::InstructionSet::Fill,
            InstructionSet::HFILL(_, _) => proto::InstructionSet::Hfill,
            InstructionSet::SFILL(_, _) => proto::InstructionSet::Sfill,
            InstructionSet::LFILL(_, _) => proto::InstructionSet::Lfill,
            InstructionSet::PAINT(_, _) => proto::InstructionSet::Paint,
            InstructionSet::HPAINT(_, _) => proto::InstructionSet::Hpaint,
            InstructionSet::SPAINT(_, _) => proto::InstructionSet::Spaint,
            InstructionSet::LPAINT(_, _) => proto::InstructionSet::Lpaint,
            InstructionSet::EFFECT(_, _, _) => proto::InstructionSet::Effect,
            InstructionSet::DELAY(_, _) => proto::InstructionSet::Delay,
            InstructionSet::PAUSE => proto::InstructionSet::Pause,
            InstructionSet::GET(_) => proto::InstructionSet::Get,
            InstructionSet::RESET => proto::InstructionSet::Reset,
        }
    }
}
