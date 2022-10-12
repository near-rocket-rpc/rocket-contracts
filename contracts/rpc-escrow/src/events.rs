use near_sdk::json_types::U128;
use near_sdk::{log, AccountId};
use near_sdk::{serde::Serialize, serde_json::json};

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum Event<'a> {
    Deposit {
        account_id: &'a AccountId,
        amount: &'a U128,
    },
    Charge {
        account_id: &'a AccountId,
        amount: &'a U128,
    },
}

impl Event<'_> {
    pub fn emit(&self) {
        let data = json!(self);
        let event_json = json!({
            "standard": "rocket-rpc",
            "version": "1.0.0",
            "event": data["event"],
            "data": [data["data"]]
        })
        .to_string();
        log!("EVENT_JSON:{}", event_json);
    }
}
