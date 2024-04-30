use anchor_lang::prelude::*;

#[account]
pub struct Election {
    pub state: u8,
    pub creator: Pubkey,
    pub winner_id: u64,
    pub winner_votes: u64,
    pub choices_len: u8,
}
impl Space for Election {
    const INIT_SPACE: usize = 8 + 1 + 32 + 8 + 1;
}

impl Election {
    pub fn record_vote(&mut self, id: u64, votes: u64) {
        if votes > self.winner_votes {
            self.winner_id = id;
            self.winner_votes = votes;
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum ElectionState {
    Inactive = 0,
    Active = 1,
    Closed = 2,
}

impl ElectionState {
    pub fn to_code(&self) -> u8 {
        match self {
            ElectionState::Inactive => 0,
            ElectionState::Active => 1,
            ElectionState::Closed => 2,
        }
    }
}
