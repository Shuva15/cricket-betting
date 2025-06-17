use anchor_lang::prelude::*;

declare_id!("F9UsdGmoyHAbp5oEqbGQNyuczUzo6fhbdMCHT9ASHmbU");

#[error_code]
pub enum ErrorCode {
    #[msg("The betting period is closed")]
    BettingClosed,
}

#[program]
pub mod cricket_betting {
    use super::*;

    pub fn create_game(ctx: Context<CreateGame>, game_id: u64) -> Result<()> {
        let game_account = &mut ctx.accounts.game_account;

        game_account.game_id = game_id;
        game_account.is_betting_open = true;
        game_account.total_bettors = 0;
        game_account.is_game_over = false;
        game_account.result_runs = None;
        game_account.result_wickets = None;
        game_account.prize_distributed = false;
        game_account.bump = ctx.bumps.game_account;

        msg!("Game account created successfully");
        Ok(())
    }

    pub fn place_bet(ctx: Context<PlaceBet>, runs: u16, wickets: u8) -> Result<()> {
        let bettor_account = &mut ctx.accounts.bettor_account;
        let game_account = &mut ctx.accounts.game_account;

        require!(game_account.is_betting_open, ErrorCode::BettingClosed);

        bettor_account.runs = runs;
        bettor_account.wickets = wickets;
        bettor_account.claimed = false;
        bettor_account.bump = ctx.bumps.bettor_account;

        game_account.total_bettors += 1;

        msg!("Bet placed");
        Ok(())
    }

    // call this instruction from backend with the help of a oracle or api when 6th over is finished (don't know if this it how it's done)
    pub fn update_betting_stop(ctx: Context<UpdateGameAccount>) -> Result<()> {
        let game_account = &mut ctx.accounts.game_account;

        game_account.is_betting_open = false;

        msg!("Betting closed");
        Ok(())
    }

    // call this instruction from backend with the match result
    pub fn update_game_result(ctx: Context<UpdateGameAccount>, runs: u16, wickets: u8) -> Result<()> {
        let game_account = &mut ctx. accounts.game_account;

        game_account.is_game_over = true;
        game_account.result_runs = Some(runs);
        game_account.result_wickets = Some(wickets);

        msg!("Result updated");
        Ok(())
    }

    pub fn distribute_prize(ctx: Context<UpdateGameAccount>) -> Result<()> {
        // this will be hard
        Ok(())
    }
}

#[account]
pub struct GameAccount {
    pub game_id: u64,
    pub is_betting_open: bool,
    pub total_bettors: u64,
    pub is_game_over: bool,
    pub result_runs: Option<u16>,
    pub result_wickets: Option<u8>,
    pub prize_distributed: bool,
    pub bump: u8,
}

#[account]
pub struct BettorAccount {
    pub runs: u16,
    pub wickets: u8,
    pub claimed: bool,
    pub bump: u8,
}

#[derive(Accounts)]
#[instruction(game_id: u64)]
pub struct CreateGame<'info> {
    #[account(init, payer = signer, space = 8 + 8 + 1 + 8 + 1 + 3 + 2 + 1 + 1, seeds = [b"game", game_id.to_le_bytes().as_ref()], bump)]
    pub game_account: Account<'info, GameAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PlaceBet<'info> {
    #[account(init, payer = signer, space = 8 + 2 + 1 + 1 + 1, seeds = [b"bettor-account", game_account.key().as_ref(), signer.key().as_ref()], bump)]
    pub bettor_account: Account<'info, BettorAccount>,
    #[account(mut, seeds= [b"game", game_account.game_id.to_le_bytes().as_ref()], bump = game_account.bump)]
    pub game_account: Account<'info, GameAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateGameAccount<'info> {
    #[account(mut)]
    pub game_account: Account<'info, GameAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
}