use anchor_lang::prelude::*;

use crate::{choices::Choice, Choices, Election};

#[derive(Accounts)]
pub struct AddChoices<'info> {
    #[account()]
    pub election: Account<'info, Election>,

    #[account(
        init,
        space=Choices::INIT_SPACE + election.choices_len as usize * Choice::space(),
        payer = signer,
        seeds=["choices".as_bytes(), election.to_account_info().key().as_ref()],
        bump,
    )]
    pub choices: Account<'info, Choices>,

    #[account(mut,
     address= election.creator,
    )]
    pub signer: Signer<'info>,

    system_program: Program<'info, System>,
}

impl<'info> AddChoices<'info> {
    pub fn add_choices(&mut self, choices: Vec<String>) -> Result<()> {
        let choices = choices
            .into_iter()
            .enumerate()
            .map(|(i, choice)| Choice {
                id: (i + 1) as u64,
                votes: 0,
                name: choice,
            })
            .collect();

        self.choices.choices = choices;
        Ok(())
    }
}
