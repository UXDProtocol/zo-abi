use anchor_lang::prelude::*;
use std::fmt;

// == ZO ==
#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Debug)]
pub enum LiquidationEvent {
    Perp,
    Spot,
}
impl fmt::Display for LiquidationEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[event]
pub struct DepositLog {
    pub col_index: u8,
    pub deposit_amount: u64,
    pub margin_key: Pubkey,
}

#[event]
pub struct WithdrawLog {
    pub col_index: u8,
    pub withdraw_amount: u64,
    pub margin_key: Pubkey,
}

#[event]
#[derive(Clone)]
pub struct LiquidationLog {
    pub liquidation_event: LiquidationEvent,
    pub base_symbol: String, // for Perp, this is the market symbol
    pub quote_symbol: Option<String>, // for Perp, None
    pub liqor_margin: Pubkey,
    pub liqee_margin: Pubkey,
    pub assets_to_liqor: i64,
    pub quote_to_liqor: i64,
}

#[event]
pub struct BankruptcyLog {
    pub base_symbol: String,
    pub liqor_margin: Pubkey,
    pub liqee_margin: Pubkey,
    pub assets_to_liqor: i64,
    pub quote_to_liqor: i64,
    pub insurance_loss: i64,
    pub socialized_loss: i64,
}

#[event]
pub struct CacheOracleNoops {
    pub symbols: Vec<String>,
}

// == DEX ==
#[event]
pub struct RealizedPnlLog {
    pub market_key: Pubkey,
    pub margin: Pubkey,
    pub is_long: bool,
    pub pnl: i64,
    pub qty_paid: i64,
    pub qty_received: i64,
}
