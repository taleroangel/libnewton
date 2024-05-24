use crate::instruction::InstructionSet;

// Transform source code into to Prism Binary Format
pub fn assemble(source: &Vec<InstructionSet>) -> Vec<u8> {
    source.iter().map::<Vec<u8>, _>(move |&x| x.into()).flatten().collect()
}