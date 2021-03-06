use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

use ics20_swap_client::msg::{
    AllowMsg, ChannelResponse, ClaimTokensMsg, CreateLockupMsg, ExecuteMsg, ExitPoolMsg,
    ExternalTokenMsg, InitMsg, JoinPoolMsg, ListChannelsResponse, LockTokensMsg, QueryMsg, SwapMsg,
    TransferMsg, UnlockTokensMsg,
};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InitMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(TransferMsg), &out_dir);
    export_schema(&schema_for!(SwapMsg), &out_dir);
    export_schema(&schema_for!(JoinPoolMsg), &out_dir);
    export_schema(&schema_for!(ExitPoolMsg), &out_dir);
    export_schema(&schema_for!(CreateLockupMsg), &out_dir);
    export_schema(&schema_for!(LockTokensMsg), &out_dir);
    export_schema(&schema_for!(UnlockTokensMsg), &out_dir);
    export_schema(&schema_for!(ClaimTokensMsg), &out_dir);
    export_schema(&schema_for!(AllowMsg), &out_dir);
    export_schema(&schema_for!(ExternalTokenMsg), &out_dir);
    export_schema(&schema_for!(ChannelResponse), &out_dir);
    export_schema(&schema_for!(ListChannelsResponse), &out_dir);
}
