use governance_types::types::Settlement;
use governance_types::types::ResolvedResult;
use governance_types::types::StatusResponse;
use governance_types::types::ConfigResponse;
use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

use governance_types::types::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("../schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(ConfigResponse), &out_dir);
    export_schema(&schema_for!(StatusResponse), &out_dir);
    export_schema(&schema_for!(ResolvedResult), &out_dir);
    export_schema(&schema_for!(Settlement), &out_dir);
}
