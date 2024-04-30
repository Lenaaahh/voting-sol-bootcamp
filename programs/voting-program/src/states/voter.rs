use anchor_lang::prelude::*;

#[account]
pub struct Voter {
    pub choice_id: u64,
}

impl Space for Voter {
    const INIT_SPACE: usize = 8 + 8;
}
