use anchor_lang::prelude::*;

#[account]
pub struct Choices {
    pub choices: Vec<Choice>,
}

impl Space for Choices {
    const INIT_SPACE: usize = 8;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub struct Choice {
    pub id: u64,
    pub votes: u64,
    pub name: String,
}

impl Choice {
    pub fn space() -> usize {
        8 + 8 + 4 + 35 // 35 chars in name
    }
}
