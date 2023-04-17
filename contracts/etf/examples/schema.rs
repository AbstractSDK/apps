use abstract_etf_app::msg::StateResponse;
use cosmwasm_schema::{export_schema, remove_schemas, schema_for};
use std::env::current_dir;
use std::fs::create_dir_all;
use abstract_etf_app::contract::EtfApp;

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    #[cfg(feature = "schema")]
    EtfApp::export_schema(&out_dir);
    export_schema(&schema_for!(StateResponse), &out_dir);
}
