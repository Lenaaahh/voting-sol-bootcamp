use anchor_lang::prelude::*;

use crate::errors::Errors;
use crate::{Choices, Election, Voter};

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub election: Account<'info, Election>,

    #[account(mut)]
    pub choices: Account<'info, Choices>,

    #[account(
        init,
        payer=signer,
        space=8+8,
        seeds=[
            b"voter",
            signer.key().as_ref(),
            election.key().as_ref()
        ],
        bump
    )]
    pub voter: Account<'info, Voter>,

    #[account(mut)]
    pub signer: Signer<'info>,

    system_program: Program<'info, System>,
}

impl<'info> Vote<'info> {
    pub fn vote(&mut self, choice_id: u64) -> Result<()> {
        let choice = self.choices.choices.iter_mut().find(|c| c.id == choice_id);

        if let Some(choice) = choice {
            choice.votes += 1;
            self.election.record_vote(choice_id, choice.votes);
        } else {
            return Err(Errors::InvalidChoice.into());
        }

        self.voter.choice_id = choice_id;
        Ok(())
    }
}
