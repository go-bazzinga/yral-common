use candid::Principal;
use serde::{Deserialize, Serialize};
use types::delegated_identity::DelegatedIdentityWire;

#[derive(Serialize, Deserialize)]
pub struct ClaimTokensRequest {
    pub identity: DelegatedIdentityWire,
    pub token_root: Principal,
}

#[derive(Serialize, Deserialize)]
pub struct ParticipateInSwapRequest {
    pub user_principal: Principal,
    pub token_root: Principal,
}
