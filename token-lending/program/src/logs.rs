#![allow(missing_docs)]
use crate::math::Decimal;
use solana_program::pubkey::Pubkey;
use std::fmt;

extern crate serde;
extern crate serde_json;

#[derive(Debug, Serialize)]
pub enum LogEventType {
    ObligationStateUpdate,
    ProgramVersion,
    PythError,
    PythOraclePriceUpdate,
    ReserveStateUpdate,
    SwitchboardError,
    SwitchboardV1OraclePriceUpdate,
}

impl fmt::Display for LogEventType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[macro_export]
macro_rules! emit_log_event {
    ($e:expr) => {
        msg!("solend-event-log:");
        msg!(&serde_json::to_string($e).unwrap());
    };
}

#[derive(Serialize)]
pub struct PythOraclePriceUpdate {
    pub event_type: LogEventType,
    pub oracle_pubkey: Pubkey,
    pub price: Decimal,
    pub confidence: u64,
    pub published_slot: u64,
}

#[derive(Serialize)]
pub struct PythError {
    pub event_type: LogEventType,
    pub oracle_pubkey: Pubkey,
    pub error_message: String,
}

#[derive(Serialize)]
pub struct SwitchboardV1OraclePriceUpdate {
    pub event_type: LogEventType,
    pub oracle_pubkey: Pubkey,
    pub price: Decimal,
    pub published_slot: u64,
}

#[derive(Serialize)]
pub struct SwitchboardError {
    pub event_type: LogEventType,
    pub oracle_pubkey: Pubkey,
    pub error_message: String,
}

#[derive(Serialize)]
pub struct ProgramVersion {
    pub event_type: LogEventType,
    pub version: u8,
}

#[derive(Serialize)]
pub struct ReserveStateUpdate {
    pub event_type: LogEventType,
    pub reserve_id: Pubkey,
    pub available_amount: u64,
    pub borrowed_amount_wads: Decimal,
    pub cumulative_borrow_rate_wads: Decimal,
    pub collateral_mint_total_supply: u64,
    pub collateral_exchange_rate: String,
}

#[derive(Serialize)]
pub struct ObligationStateUpdate {
    pub event_type: LogEventType,
    pub obligation_id: Pubkey,
    pub allowed_borrow_value: Decimal,
    pub unhealthy_borrow_value: Decimal,
    pub deposits: Vec<DepositLog>,
    pub borrows: Vec<BorrowLog>,
}

#[derive(Serialize)]
pub struct DepositLog {
    pub reserve_id: Pubkey,
    pub deposited_amount: u64,
    pub market_value: Decimal,
}

#[derive(Serialize)]
pub struct BorrowLog {
    pub reserve_id: Pubkey,
    pub cumulative_borrow_rate_wads: Decimal,
    pub borrowed_amount_wads: Decimal,
    pub market_value: Decimal,
}
