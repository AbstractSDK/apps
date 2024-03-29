//! # Liquidity Interface Add-On
//!
//! `crate::msg` is an app which allows users to deposit into or withdraw from a [`crate::proxy`] contract.
//!
//! ## Description
//! This contract uses the proxy's value calculation configuration to get the value of the assets held in the proxy and the relative value of the deposit asset.
//! It then mints LP tokens that are claimable for an equal portion of the proxy assets at a later date.
//!
//! ---
//! **WARNING:** This mint/burn mechanism can be mis-used by flash-loan attacks if the assets contained are of low-liquidity compared to the etf's size.
//!
//! ## Creation
//! The etf contract can be added on an OS by calling [`ExecuteMsg::InstallModule`](crate::manager::ExecuteMsg::InstallModule) on the manager of the os.
//! ```ignore
//! let etf_init_msg = InstantiateMsg{
//!                deposit_asset: "juno".to_string(),
//!                base: BaseInstantiateMsg{ans_host_address: "juno1...".to_string()},
//!                fee: Decimal::percent(10),
//!                manager_addr: "juno1...".to_string(),
//!                token_code_id: 3,
//!                etf_lp_token_name: Some("demo_etf".to_string()),
//!                etf_lp_token_symbol: Some("DEMO".to_string()),
//!        };
//! let create_module_msg = ExecuteMsg::InstallModule {
//!                 module: Module {
//!                     info: ModuleInfo {
//!                         name: ETF.into(),
//!                         version: None,
//!                     },
//!                     kind: crate::core::modules::ModuleKind::External,
//!                 },
//!                 init_msg: Some(to_binary(&etf_init_msg).unwrap()),
//!        };
//! // Call create_module_msg on manager
//! ```
//!
//! ## Migration
//! Migrating this contract is done by calling `ExecuteMsg::Upgrade` on [`crate::manager`] with `crate::ETF` as module.
use abstract_core::app;
use abstract_sdk::base::{ExecuteEndpoint, InstantiateEndpoint, MigrateEndpoint, QueryEndpoint};
use cosmwasm_schema::QueryResponses;
use cosmwasm_std::Decimal;
use cw_asset::AssetUnchecked;

use crate::contract::EtfApp;

pub type InstantiateMsg = <EtfApp as InstantiateEndpoint>::InstantiateMsg;
pub type ExecuteMsg = <EtfApp as ExecuteEndpoint>::ExecuteMsg;
pub type QueryMsg = <EtfApp as QueryEndpoint>::QueryMsg;
pub type MigrateMsg = <EtfApp as MigrateEndpoint>::MigrateMsg;

impl app::AppExecuteMsg for EtfExecuteMsg {}
impl app::AppQueryMsg for EtfQueryMsg {}

/// Init msg
#[cosmwasm_schema::cw_serde]
pub struct EtfInstantiateMsg {
    /// Code-id used to create the LP token
    pub token_code_id: u64,
    /// Fee charged on withdrawal
    pub fee: Decimal,
    /// Address of the ETFs manager which receives the fee.
    pub manager_addr: String,
    /// Name of the etf token
    pub token_name: Option<String>,
    /// Symbol of the etf token
    pub token_symbol: Option<String>,
}

#[cosmwasm_schema::cw_serde]
#[cfg_attr(feature = "boot", derive(boot_core::ExecuteFns))]
#[cfg_attr(feature = "boot", impl_into(ExecuteMsg))]
pub enum EtfExecuteMsg {
    /// Deposit asset into the ETF
    #[cfg_attr(feature = "boot", payable)]
    Deposit { asset: AssetUnchecked },
    /// Set the withdraw fee
    SetFee { fee: Decimal },
}

#[cosmwasm_schema::cw_serde]
#[cfg_attr(feature = "boot", derive(boot_core::QueryFns))]
#[cfg_attr(feature = "boot", impl_into(QueryMsg))]
#[derive(QueryResponses)]
pub enum EtfQueryMsg {
    // Add dapp-specific queries here
    /// Returns [`StateResponse`]
    #[returns(StateResponse)]
    State {},
}

#[cosmwasm_schema::cw_serde]
pub enum Cw20HookMsg {
    Deposit {},
    Claim {},
}

#[cosmwasm_schema::cw_serde]
pub struct StateResponse {
    pub share_token_address: String,
    pub fee: Decimal,
}
