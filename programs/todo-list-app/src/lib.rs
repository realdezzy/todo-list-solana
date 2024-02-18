use anchor_lang::prelude::*;

declare_id!("584iRz9qDp4bgkSh15xAz37PSokmd5xUR8V1ueL9sm5J");

const DISCRIMINATOR: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const BOOL_LENGTH: usize = 1;
const TEXT_LENGTH: usize = 4 + 400 * 4; // 400 chars
const TIMESTAMP_LENGTH: usize = 8;


#[program]
pub mod todo_list_app {
    use super::*;

    pub fn adding_task(
        ctx: Context<AddingTask>,
        text: String) -> Result<()> {

            let task = &mut ctx.accounts.task;
            let author = &ctx.accounts.author;

            let clock = Clock::get().unwrap();
            if text.chars().count() > 400 {
                return Err(ErrorCode::TextTooLong.into());
            }

            task.author = *author.key;
            task.is_done = false;
            task.created_at = clock.unix_timestamp;
            task.text = text;
            Ok(())
        }

    pub fn updateting_task(ctx: Context<UpdatingTask>, is_done: bool) -> Result<()> {
        let task = &mut ctx.accounts.task;
        task.is_done = is_done;
        task.updated_at = Clock::get()?.unix_timestamp;
        Ok(())
    }
}

#[account]
pub struct Task {
    pub author: Pubkey, // The account that owns the task
    pub is_done: bool, // Whether the task is done or not
    pub text: String, // The text of the task
    pub created_at: i64, // The timestamp when the task was created
    pub updated_at: i64, // The timestamp when the task was last updated
}

impl Task {
    const LEN: usize = DISCRIMINATOR +
    PUBLIC_KEY_LENGTH +
    BOOL_LENGTH +
    TEXT_LENGTH +
    TIMESTAMP_LENGTH +
    TIMESTAMP_LENGTH;
}

#[derive(Accounts)]
pub struct AddingTask<'info> {
    #[account(init, payer = author, space = Task::LEN)]
    pub task: Account<'info, Task>,
    #[account(mut)]
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdatingTask<'info> {
    #[account(mut)]
    pub task: Account<'info, Task>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("The test is too long")]
    TextTooLong,
}