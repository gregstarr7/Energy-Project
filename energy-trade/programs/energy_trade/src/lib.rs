use anchor_lang::prelude::*;

declare_id!("Dxt7cdAgWNnnEFeyJSihnhLkLjfS18j2anyhYfwXEXnV");

#[program]
pub mod energy_trade {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // Do something simple, like log or save to a dummy account if needed
        msg!("Initialized!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
