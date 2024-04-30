use anchor_lang::prelude::*;

use crate::{election::ElectionState, Election};

#[derive(Accounts)]
#[instruction(options_len:u8)]
pub struct CreateElection<'info> {
    #[account(
        init,
        payer=signer,
        space=Election::INIT_SPACE + 4 + options_len as usize *  8
    )]
    pub election: Account<'info, Election>,

    #[account(mut)]
    pub signer: Signer<'info>,

    system_program: Program<'info, System>,
}

impl<'info> CreateElection<'info> {
    pub fn create_election(&mut self, choices_len: u8) -> Result<()> {
        self.election.set_inner(Election {
            state: ElectionState::Inactive.to_code(),
            creator: self.signer.key(),
            winner_id: 0,
            winner_votes: 0,
            choices_len,
        });

        Ok(())
    }
}
