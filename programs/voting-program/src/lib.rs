use anchor_lang::prelude::*;

pub mod contexts;
pub mod states;
use contexts::*;
use states::*;
pub mod errors;

declare_id!("DsPwHpQfKWsgzGjpnW8i9c8RQtmLtxU1HRsxhBtHT1w2");

#[program]
pub mod voting_program {
    use crate::errors::Errors;

    use super::*;

    pub fn create_election(ctx: Context<CreateElection>, choises_len: u8) -> Result<()> {
        ctx.accounts.create_election(choises_len)
    }

    pub fn add_choices(ctx: Context<AddChoices>, choices: Vec<String>) -> Result<()> {
        require!(
            choices.len() == ctx.accounts.election.choices_len as usize,
            Errors::InvalidChoicesLength
        );

        ctx.accounts.add_choices(choices)
    }

    pub fn update_election_state(ctx: Context<UpdateElectionState>, state: u8) -> Result<()> {
        ctx.accounts.update_election_state(state)
    }

    pub fn vote(ctx: Context<Vote>, choice_id: u64) -> Result<()> {
        ctx.accounts.vote(choice_id)
    }
}
