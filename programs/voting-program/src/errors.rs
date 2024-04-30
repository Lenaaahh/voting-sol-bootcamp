use anchor_lang::error_code;

#[error_code]
pub enum Errors {
    #[msg("Invalid choices length")]
    InvalidChoicesLength,
    #[msg("Invalid choice")]
    InvalidChoice,
}
