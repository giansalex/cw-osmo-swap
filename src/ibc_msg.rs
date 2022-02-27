use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::ContractError;
use cosmwasm_std::{Binary, Uint128, Uint64};

/// Swap Packet
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SwapPacket {
    pub sender: String,
    pub routes: Vec<SwapAmountInRoute>,
    pub token_out_min_amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SwapAmountInRoute {
    pub pool_id: Uint64,
    pub token_out_denom: String,
}

/// This is the success response we send on ack for PacketMsg::Balance.
/// Just acknowledge success or error
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SwapAmountInAck {
    pub amount: Uint128,
    pub denom: String,
}

/// The format for sending an ics20 packet.
/// Proto defined here: https://github.com/cosmos/ibc-go/blob/v2.0.2/proto/ibc/applications/transfer/v2/packet.proto#L10-L19
/// This is compatible with the JSON serialization
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct Ics20Packet {
    /// amount of tokens to transfer is encoded as a string, but limited to u64 max
    pub amount: Uint128,
    /// the token denomination to be transferred
    pub denom: String,
    /// the recipient address on the destination chain
    pub receiver: String,
    /// the sender address
    pub sender: String,

    /// Action packet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<OsmoPacket>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum OsmoPacket {
    Swap(SwapPacket),
}

impl Ics20Packet {
    pub fn new<T: Into<String>>(
        amount: Uint128,
        denom: T,
        sender: &str,
        receiver: &str,
        action_packet: Option<OsmoPacket>,
    ) -> Self {
        Ics20Packet {
            denom: denom.into(),
            amount,
            sender: sender.to_string(),
            receiver: receiver.to_string(),
            action: action_packet,
        }
    }

    pub fn validate(&self) -> Result<(), ContractError> {
        if self.amount.u128() > (u64::MAX as u128) {
            Err(ContractError::AmountOverflow {})
        } else {
            Ok(())
        }
    }
}

/// This is a generic ICS acknowledgement format.
/// Proto defined here: https://github.com/cosmos/cosmos-sdk/blob/v0.42.0/proto/ibc/core/channel/v1/channel.proto#L141-L147
/// This is compatible with the JSON serialization
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Ics20Ack {
    Result(Binary),
    Error(String),
}

pub struct Voucher {
    pub denom: String,
    /// denom is from source chain.
    pub our_chain: bool,
}
