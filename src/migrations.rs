pub mod v1 {
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};

    use cw_storage_plus::Item;

    #[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
    pub struct Config {
        pub default_timeout: u64,
        pub init_channel: bool,
    }

    pub const CONFIG: Item<Config> = Item::new("ics20_config");
}