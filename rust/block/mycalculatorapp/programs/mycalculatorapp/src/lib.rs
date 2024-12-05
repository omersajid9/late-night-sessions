use anchor_lang::prelude::*;

declare_id!("7uKxfCFQ2GEop4fguSKbnKATXZjC2QkKTAZbCDDnP1Jc");

#[program]
pub mod mycalculatorapp {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
