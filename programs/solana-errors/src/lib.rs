#![allow(clippy::result_large_err)]
use anchor_lang::prelude::*;

declare_id!("Gp3jcr7dqCcgp3QbQdcwjS5p5n5usRLoxesQuNaHm4GD");

#[program]
pub mod solana_errors {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, count: u8) -> Result<()> {
        let data: &mut Account<MyData> = &mut ctx.accounts.data;

        require!(count <= 10, MyError::InvalidCount);

        data.authority = ctx.accounts.user.key();
        data.counter = math_function(count).unwrap();

        msg!("data.conter = {}", data.counter);
        msg!("data pubkey = {}", data.key().to_string());
        msg!("user pubkey = {}", data.authority.key().to_string());

        Ok(())
    }
}

fn math_function(count: u8) -> Option<u8> {
    10u8.checked_sub(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_number_success() {
        assert_eq!(math_function(2), Some(8));
    }

    #[test]
    fn test_limit_number_success() {
        assert_eq!(math_function(10), Some(0));
    }

    #[test]
    fn test_big_number_fail() {
        assert_eq!(math_function(11), None);
    }
}


#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    user: Signer<'info>,

    #[account(init,
        space = 8 + 32 + 1,  //  8 bytes for DESCRIMINANT + 32 bytes for the PubKey + 1 byte for counter
        payer = user,
        seeds = [b"data"],
        bump
    )]
    data: Account<'info, MyData>,

    system_program: Program<'info, System>,
}

#[account]
pub struct MyData {
    authority: Pubkey,
    counter: u8,
}

#[error_code]
pub enum MyError {
    #[msg("Invalid count value")]
    InvalidCount
}