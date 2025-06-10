use anchor_lang::prelude::*;

declare_id!("F9UsdGmoyHAbp5oEqbGQNyuczUzo6fhbdMCHT9ASHmbU");

#[program]
pub mod cricket_betting {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
