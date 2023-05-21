use anchor_lang::prelude::*;
use instructions::*;

declare_id!("FsgyMvD4vw6xSMNkFD14gbgRK5kadrZYzF1xGAcj2WfR");

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;

#[program]
pub mod todo {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize(ctx)
    }

    // pub fn create_todo(ctx: Context<CreateTodo>, content: String) -> Result<()> {
    //     instructions::create_todo(ctx, content)
    // }

    // pub fn delete_todo(ctx: Context<DeleteTodo>, id: Pubkey) -> Result<()> {
    //     instructions::delete_todo(ctx, id)
    // }

    // pub fn update_todo(ctx: Context<UpdateTodo>, todo: Todo) -> Result<()> {
    //     instructions::update_todo(ctx, todo)
    // }
}
