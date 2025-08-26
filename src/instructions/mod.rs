use pinocchio::program_error::ProgramError;

pub mod deposit;
pub mod initialize;
pub mod swap;
pub mod withdraw;

pub use deposit::*;
pub use initialize::*;
pub use swap::*;
pub use withdraw::*;

#[repr(u8)]
pub enum ProgramInstruction {
    Initialize,
    Deposit,
    Withdraw,
    Swap,
}

impl TryFrom<&u8> for ProgramInstruction {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match *value {
            0 => Ok(ProgramInstruction::Initialize),
            1 => Ok(ProgramInstruction::Deposit),
            2 => Ok(ProgramInstruction::Withdraw),
            3 => Ok(ProgramInstruction::Swap),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
