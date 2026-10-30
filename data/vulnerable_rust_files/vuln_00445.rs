// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2024-0385
// CVE: N/A
// CWE: N/A
// Crate: cw0
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/cw0
// Extraction Method: repo_search
// GitHub Repo: CosmWasm/cw-plus
// Commit: 3854e26
// File: contracts/cw20-ics20/src/ibc.rs


use cosmwasm_std::entry_point;
 use cosmwasm_std::{
     attr, from_json, to_json_binary, BankMsg, Binary, CosmosMsg, Deps, DepsMut, Env,
    IbcBasicResponse, IbcChannel, IbcChannelCloseMsg, IbcChannelConnectMsg, IbcChannelOpenMsg,
    IbcEndpoint, IbcOrder, IbcPacket, IbcPacketAckMsg, IbcPacketReceiveMsg, IbcPacketTimeoutMsg,
    IbcReceiveResponse, Reply, Response, SubMsg, SubMsgResult, Uint128, WasmMsg,
 };
 
 use crate::amount::Amount;
     _deps: DepsMut,
     _env: Env,
     msg: IbcChannelOpenMsg,
) -> Result<(), ContractError> {
     enforce_order_and_version(msg.channel(), msg.counterparty_version())?;
    Ok(())
 }
 
 #[cfg_attr(not(feature = "library"), entry_point)]