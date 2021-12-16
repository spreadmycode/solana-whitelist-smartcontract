use anchor_lang::prelude::*;
use std::convert::TryInto;
use std::ops::DerefMut;

declare_id!("9pkYv3SvYfd51mh1v3vn1aX2WHepx7wZxsfxpmsNdFT7");

const MAX_LEN: usize = 3000;

#[program]
pub mod whitelist {

    use super::*;

    ///////////////////////////////////////////////////////////////////////////
    ///                          Smart Contract                             ///
    ///////////////////////////////////////////////////////////////////////////

    /**
     *  Initialize contract data
     */
    pub fn initialize_contract(
        ctx: Context<Initialize>
    ) -> ProgramResult {
        let data = &mut ctx.accounts.data;

        let mut whitelist_data = ctx.accounts.whitelist_data.load_init()?;
        let whitelist = whitelist_data.deref_mut();
        whitelist.data = [Pubkey::default(); MAX_LEN];

        data.counter = 0;
        data.period_status = PeriodStatus::PreSale as u8;

        Ok(())
    }

	/**
     *  Add addresses
     */
    pub fn add_whitelists(
        ctx: Context<Status>,
        addresses: Vec<Pubkey>,
    ) -> ProgramResult {
        let data = &mut ctx.accounts.data;
        let mut whitelist_data = ctx.accounts.whitelist_data.load_mut()?;
        let mut whitelist = whitelist_data.deref_mut();

        let length = addresses.len();
        let mut counter = data.counter as usize;

        // Check that new addresses don't exceed remaining space
        if length + counter > MAX_LEN {
            return Ok(());
        }

        let mut current_data: Vec<Pubkey> = Vec::new();
        // Fill with current data
        current_data.extend_from_slice(&whitelist.data[0..counter]);
        counter += length;

        // Add new addresses
        current_data.extend_from_slice(&addresses);

        // Fill remaining data with default Pubkey
        for _ in counter..MAX_LEN {
            current_data.push(Pubkey::default());
        }

        // Update data
        let new_data: [Pubkey; MAX_LEN] = match current_data.try_into() {
            Ok(data) => data,
            Err(e) => {
                msg!("Error: {:?}", e);
                return Ok(())
            }
        };
        whitelist.data = new_data;
        data.counter = counter as u64;

        Ok(())
    }

    /**
     *  Clear whitelist
     */
    pub fn clear_whitelist(
        ctx: Context<Status>,
    ) -> ProgramResult {
        let data = &mut ctx.accounts.data;
        data.counter = 0;

        Ok(())
    }

    /**
     *  Set sale is pending
     */
    pub fn set_pending(
        ctx: Context<Status>
    ) -> ProgramResult {
        let data = &mut ctx.accounts.data;
        data.period_status = PeriodStatus::PendingSale as u8;

        Ok(())
    }

    /**
     *  Toggle pre/post sale period
     */
    pub fn toggle_period(
        ctx: Context<Status>
    ) -> ProgramResult {
        let data = &mut ctx.accounts.data;

		if data.period_status == PeriodStatus::PendingSale as u8 {
            data.period_status = PeriodStatus::PreSale as u8;
        } else if data.period_status == PeriodStatus::PreSale as u8 {
            data.period_status = PeriodStatus::PostSale as u8;
        } else if data.period_status == PeriodStatus::PostSale as u8 {
            data.period_status = PeriodStatus::PreSale as u8;
        }

        Ok(())
    }

    ////////////////////////////////////////////////////////////////////////////////////
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 32 + 8)]
    pub data: ProgramAccount<'info, Data>,
    #[account(zero)]
    pub whitelist_data: AccountLoader<'info, WhitelistData>,
    pub user: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Status<'info> {
    #[account(mut)]
    pub data: ProgramAccount<'info, Data>,
    #[account(mut)]
    pub whitelist_data: AccountLoader<'info, WhitelistData>,
    pub minter: AccountInfo<'info>,
}

#[account]
pub struct Data {
    pub period_status: u8,
    pub counter: u64,
}

pub enum PeriodStatus {
    PendingSale = 0,
    PreSale     = 1,
    PostSale    = 2
}

#[account(zero_copy)]
pub struct WhitelistData {
    pub data: [Pubkey; 3000],
}