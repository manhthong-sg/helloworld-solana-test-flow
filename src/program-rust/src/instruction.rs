use solana_program::program_error::ProgramError;
use std::convert::TryInto;
#[derive(Debug)]
pub enum HelloInstruction {
    Increment,
    Decrement,
    Set(u32),
}

impl HelloInstruction {
    //ham unpack data
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;

        match tag {
            0=> return Ok(HelloInstruction::Increment),
            1=> return Ok(HelloInstruction::Decrement),
            2=> {
                if rest.len() != 4{ 
                    return Err(ProgramError::InvalidInstructionData);
                }
                let val:Result<[u8; 4], _>  = rest[..4].try_into();
                
                match val{
                    Ok(i) =>{
                        return Ok(HelloInstruction::Set(u32::from_le_bytes(i)))
                    }
                    _=>  return Err(ProgramError::InvalidInstructionData)
                }
            },
            _=> Err(ProgramError::InvalidInstructionData)
        }
    }
}
