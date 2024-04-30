use anchor_lang::prelude::*;

use crate::Election;

#[derive(Accounts)]
pub struct UpdateElectionState<'info> {
    #[account(mut)]
    pub election: Account<'info, Election>,

    #[account(mut,
      address=election.creator,
    )]
    pub signer: Signer<'info>,

    system_program: Program<'info, System>,
}

impl<'info> UpdateElectionState<'info> {
    pub fn update_election_state(&mut self, state: u8) -> Result<()> {
        self.election.state = state;
        Ok(())
    }
}
