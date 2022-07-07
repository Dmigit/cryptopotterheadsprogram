use anchor_lang::prelude::*;

declare_id!("G959YjxpNdB7Ngp1yXS1iHL2XpaaogpjhE5LBfTmXH41");

#[program]
pub mod myepicproject {
    use super::*;
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result <()> {
        //Get account reference
        let base_account = &mut ctx.accounts.base_account;
        //initialize total_gifs
        base_account.total_gifs = 0;
        Ok(())
    }

    pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> Result <()> {
        //Get a reference to the account and increment total_gifs
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        //Building the struct
        let item = ItemStruct {
            gif_link: gif_link.to_string(),
            user_address: *user.to_account_info().key,
        };
        //Add the struct to the gif_list vector
        base_account.gif_list.push(item);
        base_account.total_gifs += 1;
        Ok(())
    }
}

//Attach certain variables to the StartStuffOff context
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

//Specify what data you want in the AddGif Context
#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

//Custom struct
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
}

//Tell Solana what we want to store on this account
#[account] 
pub struct BaseAccount {
    pub total_gifs: u64,
    pub gif_list: Vec<ItemStruct>,
}