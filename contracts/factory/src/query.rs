use cosmwasm_std::{Deps, StdResult, Uint256, Binary};

use crate::{msg::CampaignsResponse, state::CAMPAIGNS};

const DEFAULT_LIMIT: u32 = 10;
const MAX_LIMIT: u32 = 30;

pub fn query_campaigns(
    deps: Deps,
    _start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<CampaignsResponse> {

    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;

    // fix start_after bound 
    // let  start_after = start_after.map(|s| s.to_string());

    let campaigns: StdResult<Vec<_>> = CAMPAIGNS
        .range(deps.storage, None, None,  cosmwasm_std::Order::Ascending)
        .take(limit)
        .collect();

    let campaigns = campaigns?;

    Ok(CampaignsResponse { campaigns } )


}