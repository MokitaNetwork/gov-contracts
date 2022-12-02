mod msg;
mod query;

pub use msg::AetherMessages;
pub use query::{
    AetherQuery, GetAppResponse, GetAssetDataResponse, MessageValidateResponse, StateResponse,
    TotalSupplyResponse,
};

// This is a signal, such that any contract that imports these helpers will only run on the
// aether blockchain
#[no_mangle]
extern "C" fn requires_aether() {}
