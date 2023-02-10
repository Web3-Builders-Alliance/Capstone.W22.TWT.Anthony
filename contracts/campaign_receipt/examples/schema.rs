use std::env::current_dir;
use std::fs::create_dir_all;

use campaign_receipt::{contract::Metadata, msg::ExecuteMsg};
use cosmwasm_schema::{export_schema, remove_schemas, schema_for};
use cw721_base::{InstantiateMsg, QueryMsg as Cw721QueryMsg};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg<Metadata>), &out_dir);
    export_schema(&schema_for!(Cw721QueryMsg<Metadata>), &out_dir);
}
