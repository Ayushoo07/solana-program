use anchor_lang::prelude::*;

declare_id!("J4wp7RGbc2HRB5S7d9bRd1kHP9SoujCmFXZLi44cxyvc"); // Unique program ID

use anchor_lang::solana_program::clock::Clock;
use anchor_lang::prelude::Pubkey;

#[program]
pub mod todo_list {
    use super::*;

    /// Initializes the to-do account by setting up an empty task list.
    pub fn initialize_todo_account(ctx: Context<InitializeTodoAccount>) -> Result<()> {
        let todo_account = &mut ctx.accounts.todo_account;
        todo_account.tasks = Vec::new();
        Ok(())
    }

    /// Creates a new to-do item and assigns a unique ID based on the current timestamp.
    pub fn create_todo_item(ctx: Context<ModifyTodoItem>, description: String, due_date: i64) -> Result<u32> {
        require!(description.len() <= 280, CustomError::DescriptionTooLong);

        let todo_account = &mut ctx.accounts.todo_account;
        let clock = Clock::get()?; // Fetch current timestamp
        let id = clock.unix_timestamp as u32; // Generate unique ID using timestamp

        todo_account.tasks.push(TodoItem {
            id,
            description,
            completed: false,
            due_date,
            owner: ctx.accounts.user.key(),
        });

        Ok(id)
    }

    /// Marks a to-do item as complete or incomplete.
    pub fn mark_todo_status(ctx: Context<ModifyTodoItem>, id: u32, completed: bool) -> Result<()> {
        let todo_account = &mut ctx.accounts.todo_account;

        // Idiomatic `.find_mut()` instead of `.iter_mut().find()`
        if let Some(task) = todo_account.tasks.iter_mut().find(|t| t.id == id) {
            task.completed = completed;
        } else {
            return Err(CustomError::InvalidTaskId.into());
        }

        Ok(())
    }

    /// Updates the description of an existing to-do item.
    pub fn update_todo_description(ctx: Context<ModifyTodoItem>, id: u32, new_description: String) -> Result<()> {
        require!(new_description.len() <= 280, CustomError::DescriptionTooLong);

        let todo_account = &mut ctx.accounts.todo_account;

        if let Some(task) = todo_account.tasks.iter_mut().find(|t| t.id == id) {
            task.description = new_description;
        } else {
            return Err(CustomError::InvalidTaskId.into());
        }

        Ok(())
    }

    /// Deletes a to-do item from the task list.
    pub fn delete_todo_item(ctx: Context<ModifyTodoItem>, id: u32) -> Result<()> {
        let todo_account = &mut ctx.accounts.todo_account;
        
        // Store initial length to check if an item was actually removed
        let initial_len = todo_account.tasks.len();
        todo_account.tasks.retain(|t| t.id != id);
        
        // If no item was removed, return an error
        if todo_account.tasks.len() == initial_len {
            return Err(CustomError::InvalidTaskId.into());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::*;
    use solana_program_test::*;
    use solana_sdk::{signer::Signer, transport::TransportError};

    /// Tests if the todo account initializes correctly.
    #[tokio::test]
    async fn test_initialize_todo_account() -> Result<(), TransportError> {
        let program = ProgramTest::new("todo_list", id(), processor!(todo_list::entry));
        let (mut banks_client, payer, recent_blockhash) = program.start().await;

        let todo_account_key = Pubkey::find_program_address(&[b"todo", payer.pubkey().as_ref()], &id()).0;

        let tx = Transaction::new_signed_with_payer(
            &[Instruction {
                program_id: id(),
                accounts: vec![
                    AccountMeta::new(todo_account_key, false),
                    AccountMeta::new(payer.pubkey(), true),
                ],
                data: todo_list::instruction::InitializeTodoAccount {},
            }],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        );

        banks_client.process_transaction(tx).await?;
        Ok(())
    }

    /// Tests the creation of a new todo item.
    #[tokio::test]
    async fn test_create_todo_item() -> Result<(), TransportError> {
        let program = ProgramTest::new("todo_list", id(), processor!(todo_list::entry));
        let (mut banks_client, payer, recent_blockhash) = program.start().await;

        let todo_account_key = Pubkey::find_program_address(&[b"todo", payer.pubkey().as_ref()], &id()).0;

        let tx = Transaction::new_signed_with_payer(
            &[Instruction {
                program_id: id(),
                accounts: vec![
                    AccountMeta::new(todo_account_key, false),
                    AccountMeta::new(payer.pubkey(), true),
                ],
                data: todo_list::instruction::CreateTodoItem {
                    description: "Test task".to_string(),
                    due_date: 1710000000,
                },
            }],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        );

        banks_client.process_transaction(tx).await?;
        Ok(())
    }
    
    /// Tests deletion of a todo item.
    #[tokio::test]
    async fn test_delete_todo_item() -> Result<(), TransportError> {
        let program = ProgramTest::new("todo_list", id(), processor!(todo_list::entry));
        let (mut banks_client, payer, recent_blockhash) = program.start().await;

        let todo_account_key = Pubkey::find_program_address(&[b"todo", payer.pubkey().as_ref()], &id()).0;

        let tx = Transaction::new_signed_with_payer(
            &[Instruction {
                program_id: id(),
                accounts: vec![
                    AccountMeta::new(todo_account_key, false),
                    AccountMeta::new(payer.pubkey(), true),
                ],
                data: todo_list::instruction::DeleteTodoItem { id: 1 },
            }],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        );

        banks_client.process_transaction(tx).await?;
        Ok(())
    }
}

/// Context for initializing the to-do account.
#[derive(Accounts)]
pub struct InitializeTodoAccount<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(init, seeds = [b"todo", user.key().as_ref()], bump, payer = user, space = 8 + 1024)]
    pub todo_account: Account<'info, TodoAccount>,

    pub system_program: Program<'info, System>,
}

/// Context for modifying a to-do item.
#[derive(Accounts)]
pub struct ModifyTodoItem<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut, seeds = [b"todo", user.key().as_ref()], bump)]
    pub todo_account: Account<'info, TodoAccount>,
}

/// The to-do account that stores tasks.
#[account]
pub struct TodoAccount {
    pub tasks: Vec<TodoItem>, // Stores all to-do items
}

/// Structure representing an individual to-do item.
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TodoItem {
    pub id: u32,            // Unique identifier for the task
    pub description: String, // Task description
    pub completed: bool,    // Task status
    pub due_date: i64,      // Due date timestamp
    pub owner: Pubkey,      // Owner of the task
}

/// Custom errors for the to-do list program.
#[error_code]
pub enum CustomError {
    #[msg("Description too long, max 280 characters.")]
    DescriptionTooLong,

    #[msg("Invalid task ID.")]
    InvalidTaskId,
}
