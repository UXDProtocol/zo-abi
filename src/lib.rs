#![allow(unused_variables)]

pub mod dex;
pub mod events;
mod types;

pub use crate::types::*;
use anchor_lang::prelude::*;

// Devnet
declare_id!("DuSPvazsfthvWRuJ8TUs984VXCeUfJ1qbzd8NwkRLEpd");

pub mod serum {
    use super::*;
    // Devnet
    declare_id!("DESVgJVGajEgKGXhb6XmqDHGz3VjdgP7rEVESBgxmroY");
}

#[program]
pub mod zo_abi {
    use super::*;

    // ========== MARGIN ==========

    pub fn create_margin(
        cx: Context<CreateMargin>,
        margin_nonce: u8
    ) -> ProgramResult { Ok(()) }

    pub fn deposit(
        cx: Context<Deposit>,
        repay_only: bool,
        amount: u64
    ) -> ProgramResult { Ok(()) }

    pub fn withdraw(
        cx: Context<Withdraw>,
        allow_borrow: bool,
        amount: u64
    ) -> ProgramResult { Ok(()) }

    // ========== TRADING ==========

    /// Creates a trader's open orders account for a given market
    pub fn create_perp_open_orders(
        cx: Context<CreatePerpOpenOrders>
    ) -> ProgramResult {Ok(())}

    /// Places a new order
    pub fn place_perp_order(
        cx: Context<PlacePerpOrder>,
        is_long: bool,
        limit_price: u64,
        max_base_quantity: u64,
        max_quote_quantity: u64,
        order_type: OrderType,
        limit: u16,
        client_id: u64,
    ) -> ProgramResult {
        Ok(())
    }

    /// Cancels an order on the book
    pub fn cancel_perp_order(
        cx: Context<CancelPerpOrder>,
        order_id: u128,
        is_long: bool,
    ) -> ProgramResult {
        Ok(())
    }

    /// Cancels an order on the book by client id
    pub fn cancel_perp_order_by_client_id(
        cx: Context<CancelPerpOrderByClientId>,
        client_id: u64,
    ) -> ProgramResult {
        Ok(())
    }

    /// Settles unrealized funding and realized pnl into the margin account
    pub fn settle_funds(cx: Context<SettleFunds>) -> ProgramResult {
       Ok(())
    }

    /// Swaps two tokens on a single A/B market, where A is the base currency
    /// and B is the quote currency. This is just a direct IOC trade that
    /// instantly settles.
    ///
    /// When side is "bid", then swaps B for A. When side is "ask", then swaps
    /// A for B.
    pub fn swap(
        cx: Context<Swap>,
        buy: bool,
        allow_borrow: bool, // whether the withdraw currency can go below 0
        amount: u64, // smol amount to swap *from*
        min_rate: u64,  // number of smol tokens received from a single big token given
    ) -> ProgramResult {
        Ok(())
    }

    // ========== KEEPERS ==========

    pub fn update_perp_funding(
        cx: Context<UpdatePerpFunding>,
    ) -> ProgramResult {
        Ok(())
    }

    pub fn cache_oracle(
        cx: Context<CacheOracle>,
        symbols: Vec<String>,
        mock_prices: Option<Vec<Option<u64>>>,
    ) -> ProgramResult {
        Ok(())
    }

    pub fn cache_interest_rates(
        cx: Context<CacheInterestRates>,
        start: u8,
        end: u8,
    ) -> ProgramResult {
        Ok(())
    }

    pub fn consume_events(
        cx: Context<ConsumeEvents>,
        limit: u16,
    ) -> ProgramResult {
        Ok(())
    }

    pub fn crank_pnl(cx: Context<CrankPnl>) -> ProgramResult {
        Ok(())
    }

    // ========== LIQUIDATION ==========

    /// Force cancels all orders of an account under liquidation
    pub fn force_cancel_all_perp_orders(
        cx: Context<ForceCancelAllPerpOrders>,
        limit: u16,
    ) -> ProgramResult {
        Ok(())
    }

    /// Liquidates a perp position by transferring it from the liqee to the liqor
    pub fn liquidate_perp_position(
        cx: Context<LiquidatePerpPosition>,
        asset_transfer_lots: u64,
    ) -> ProgramResult {
        Ok(())
    }

    /// Liquidates a spot position by transferring it from the liqee to the liqor
    pub fn liquidate_spot_position(
        cx: Context<LiquidateSpotPosition>,
        asset_transfer_amount: i64,
    ) -> ProgramResult {
        Ok(())
    }

    /// Transfer negative borrows from liqee to liqor, and subsidize through insurance fund
    pub fn settle_bankruptcy(cx: Context<SettleBankruptcy>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SettleFunds<'info> {
    pub authority: Signer<'info>,
    pub state: AccountLoader<'info, State>,
    #[account(mut)]
    pub state_signer: UncheckedAccount<'info>,
    #[account(mut)]
    pub cache: AccountLoader<'info, Cache>,
    #[account(mut)]
    pub margin: AccountLoader<'info, Margin>,
    #[account(mut)]
    pub control: AccountLoader<'info, Control>,
    #[account(mut)]
    pub open_orders: UncheckedAccount<'info>,
    #[account(mut)]
    pub dex_market: UncheckedAccount<'info>,
    pub dex_program: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct CancelPerpOrderByClientId<'info> {
    pub state: AccountLoader<'info, State>,
    #[account(mut)]
    pub cache: AccountLoader<'info, Cache>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub margin: AccountLoader<'info, Margin>,
    #[account(mut)]
    pub control: AccountLoader<'info, Control>,
    #[account(mut)]
    pub open_orders: UncheckedAccount<'info>,
    #[account(mut)]
    pub dex_market: UncheckedAccount<'info>,
    #[account(mut)]
    pub market_bids: UncheckedAccount<'info>,
    #[account(mut)]
    pub market_asks: UncheckedAccount<'info>,
    #[account(mut)]
    pub event_q: UncheckedAccount<'info>,
    pub dex_program: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct CancelPerpOrder<'info> {
    pub state: AccountLoader<'info, State>,
    #[account(mut)]
    pub cache: AccountLoader<'info, Cache>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub margin: AccountLoader<'info, Margin>,
    #[account(mut)]
    pub control: AccountLoader<'info, Control>,
    #[account(mut)]
    pub open_orders: UncheckedAccount<'info>,
    #[account(mut)]
    pub dex_market: UncheckedAccount<'info>,
    #[account(mut)]
    pub market_bids: UncheckedAccount<'info>,
    #[account(mut)]
    pub market_asks: UncheckedAccount<'info>,
    #[account(mut)]
    pub event_q: UncheckedAccount<'info>,
    pub dex_program: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct CreatePerpOpenOrders<'info> {
    pub state: AccountLoader<'info, State>,
    #[account(mut)]
    pub state_signer: UncheckedAccount<'info>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub margin: AccountLoader<'info, Margin>,
    #[account(mut)]
    pub control: AccountLoader<'info, Control>,
    #[account(mut)]
    pub open_orders: UncheckedAccount<'info>,
    #[account(mut)]
    pub dex_market: UncheckedAccount<'info>,
    pub dex_program: UncheckedAccount<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateMargin<'info> {
    pub state: AccountInfo<'info>,
    pub authority: Signer<'info>,
    /// Must be an uninitialized Keypair with
    /// ` seeds = [authority.key.as_ref(), state.key().as_ref(), b"marginv1".as_ref()] `
    #[account(mut)]
    pub margin: AccountInfo<'info>,
    /// The control account must be created as a pre-instruction, with the correct size, and with
    /// the zo program as the owner. Current size is 8 + 4482
    #[account(zero)]
    pub control: AccountInfo<'info>,
    pub rent: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    pub state: AccountInfo<'info>,
    /// ` seeds = [state.key().as_ref()] `
    pub state_signer: AccountInfo<'info>,
    #[account(mut)]
    pub cache: AccountInfo<'info>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub margin: AccountInfo<'info>,
    #[account(mut)]
    pub token_account: AccountInfo<'info>,
    /// Vault pubkey can be found from the State account
    #[account(mut)]
    pub vault: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub state: AccountInfo<'info>,
    /// ` seeds = [state.key().as_ref()] `
    #[account(mut)]
    pub state_signer: AccountInfo<'info>,
    #[account(mut)]
    pub cache: AccountInfo<'info>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub margin: AccountInfo<'info>,
    #[account(mut)]
    pub control: AccountInfo<'info>,
    #[account(mut)]
    pub token_account: AccountInfo<'info>,
    /// Vault pubkey can be found from the State account
    #[account(mut)]
    pub vault: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdatePerpFunding<'info> {
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub state_signer: AccountInfo<'info>,
    #[account(mut)]
    pub cache: AccountInfo<'info>,
    #[account(mut)]
    pub dex_market: AccountInfo<'info>,
    #[account(mut)]
    pub market_bids: AccountInfo<'info>,
    #[account(mut)]
    pub market_asks: AccountInfo<'info>,
    #[account(address = dex::ID)]
    pub dex_program: AccountInfo<'info>,
}

/// Price info accounts are passed in remaining
/// accounts array.
#[derive(Accounts)]
pub struct CacheOracle<'info> {
    pub signer: Signer<'info>,
    #[account(mut)]
    pub cache: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CacheInterestRates<'info> {
    pub signer: Signer<'info>,
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub cache: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ConsumeEvents<'info> {
    #[account(mut)]
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub state_signer: AccountInfo<'info>,
    pub dex_program: AccountInfo<'info>,
    #[account(mut)]
    pub market: AccountInfo<'info>,
    #[account(mut)]
    pub event_queue: AccountInfo<'info>,
    // RA: [alice_control, bob_control, ..., alice_oo, bob_oo, ...]
}

#[derive(Accounts)]
pub struct CrankPnl<'info> {
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub state_signer: AccountInfo<'info>,
    #[account(mut)]
    pub cache: AccountInfo<'info>,
    pub dex_program: AccountInfo<'info>,
    #[account(mut)]
    pub market: AccountInfo<'info>,
    // RA: [alice_control, bob_control, ..., alice_oo, bob_oo, ..., alice_margin, bob_margin, ...]
}

#[derive(Accounts)]
pub struct ForceCancelAllPerpOrders<'info> {
    pub pruner: Signer<'info>,
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub cache: AccountInfo<'info>,
    #[account(mut)]
    pub state_signer: AccountInfo<'info>,
    #[account(mut)]
    pub liqee_margin: AccountInfo<'info>,
    #[account(mut)]
    pub liqee_control: AccountInfo<'info>,
    #[account(mut)]
    pub liqee_oo: AccountInfo<'info>,
    #[account(mut)]
    pub dex_market: AccountInfo<'info>,
    #[account(mut)]
    pub req_q: AccountInfo<'info>,
    #[account(mut)]
    pub event_q: AccountInfo<'info>,
    #[account(mut)]
    pub market_bids: AccountInfo<'info>,
    #[account(mut)]
    pub market_asks: AccountInfo<'info>,
    #[account(address = dex::ID)]
    pub dex_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct LiquidatePerpPosition<'info> {
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub cache: AccountInfo<'info>,
    #[account(mut)]
    pub state_signer: AccountInfo<'info>,
    pub liqor: Signer<'info>,
    #[account(mut)]
    pub liqor_margin: AccountInfo<'info>,
    #[account(mut)]
    pub liqor_control: AccountInfo<'info>,
    #[account(mut)]
    pub liqor_oo: AccountInfo<'info>,
    pub liqee: AccountInfo<'info>,
    #[account(mut)]
    pub liqee_margin: AccountInfo<'info>,
    #[account(mut)]
    pub liqee_control: AccountInfo<'info>,
    #[account(mut)]
    pub liqee_oo: AccountInfo<'info>,
    #[account(mut)]
    pub dex_market: AccountInfo<'info>,
    #[account(mut)]
    pub req_q: AccountInfo<'info>,
    #[account(mut)]
    pub event_q: AccountInfo<'info>,
    #[account(mut)]
    pub market_bids: AccountInfo<'info>,
    #[account(mut)]
    pub market_asks: AccountInfo<'info>,
    #[account(address = dex::ID)]
    pub dex_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct LiquidateSpotPosition<'info> {
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub cache: AccountInfo<'info>,
    pub liqor: Signer<'info>,
    #[account(mut)]
    pub liqor_margin: AccountInfo<'info>,
    #[account(mut)]
    pub liqor_control: AccountInfo<'info>,
    #[account(mut)]
    pub liqee_margin: AccountInfo<'info>,
    #[account(mut)]
    pub liqee_control: AccountInfo<'info>,
    pub asset_mint: AccountInfo<'info>,
    pub quote_mint: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct PlacePerpOrder<'info> {
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub state_signer: AccountInfo<'info>,
    #[account(mut)]
    pub cache: AccountInfo<'info>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub margin: AccountInfo<'info>,
    #[account(mut)]
    pub control: AccountInfo<'info>,
    #[account(mut)]
    pub open_orders: AccountInfo<'info>,
    #[account(mut)]
    pub dex_market: AccountInfo<'info>,
    #[account(mut)]
    pub req_q: AccountInfo<'info>,
    #[account(mut)]
    pub event_q: AccountInfo<'info>,
    #[account(mut)]
    pub market_bids: AccountInfo<'info>,
    #[account(mut)]
    pub market_asks: AccountInfo<'info>,
    #[account(address = dex::ID)]
    pub dex_program: AccountInfo<'info>,
    pub rent: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SettleBankruptcy<'info> {
    #[account(mut)]
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub state_signer: AccountInfo<'info>,
    #[account(mut)]
    pub cache: AccountInfo<'info>,
    pub liqor: Signer<'info>,
    #[account(mut)]
    pub liqor_margin: AccountInfo<'info>,
    #[account(mut)]
    pub liqor_control: AccountInfo<'info>,
    #[account(mut)]
    pub liqee_margin: AccountInfo<'info>,
    #[account(mut)]
    pub liqee_control: AccountInfo<'info>,
    pub asset_mint: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Swap<'info> {
    pub authority: Signer<'info>,
    #[account(mut)]
    pub state: AccountInfo<'info>,
    pub state_signer: AccountInfo<'info>,
    #[account(mut)]
    pub cache: AccountInfo<'info>,
    #[account(mut)]
    pub margin: AccountInfo<'info>,
    #[account(mut)]
    pub control: AccountInfo<'info>,
    /// For the dex, this is the _quote_ mint. However, for zo, quote
    /// refers to the collateral at index 0, normally USDC.
    pub quote_mint: AccountInfo<'info>,
    #[account(mut)]
    pub quote_vault: AccountInfo<'info>,
    pub asset_mint: AccountInfo<'info>,
    #[account(mut)]
    pub asset_vault: AccountInfo<'info>,
    #[account(mut)]
    pub swap_fee_vault: AccountInfo<'info>,
    #[account(mut)]
    pub serum_open_orders: AccountInfo<'info>,
    #[account(mut)]
    pub serum_market: AccountInfo<'info>,
    #[account(mut)]
    pub serum_request_queue: AccountInfo<'info>,
    #[account(mut)]
    pub serum_event_queue: AccountInfo<'info>,
    #[account(mut)]
    pub serum_bids: AccountInfo<'info>,
    #[account(mut)]
    pub serum_asks: AccountInfo<'info>,
    #[account(mut)]
    pub serum_coin_vault: AccountInfo<'info>,
    #[account(mut)]
    pub serum_pc_vault: AccountInfo<'info>,
    pub serum_vault_signer: AccountInfo<'info>,
    pub srm_spot_program: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub rent: AccountInfo<'info>,
}
