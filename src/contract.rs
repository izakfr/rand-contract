#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use crate::errors::{ExecuteError, QueryError};
use crate::msg::{
    BountiesResponse, Bounty, ExecuteMsg, GetResponse, InstantiateMsg, LatestResponse, QueryMsg,
};
use crate::state::{
    beacons_storage, beacons_storage_read, bounties_storage, bounties_storage_read, config,
    config_read, Config,
};
use cosmwasm_std::{to_binary, BankMsg, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Order, Response, StdResult, Storage, coins, SubMsg, ReplyOn};
use drand_verify::{derive_randomness, g1_from_variable, verify};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    config(deps.storage).save(&Config {
        pubkey: msg.pubkey,
        bounty_denom: msg.bounty_denom,
    })?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ExecuteError> {
    match msg {
        ExecuteMsg::SetBounty { round } => try_set_bounty(deps, info, round),
        ExecuteMsg::Add {
            round,
            previous_signature,
            signature,
        } => try_add(deps, info, round, previous_signature, signature),
    }
}

pub fn try_set_bounty(
    deps: DepsMut,
    info: MessageInfo,
    round: u64,
) -> Result<Response, ExecuteError> {
    let denom = config_read(deps.storage).load()?.bounty_denom;

    let matching_coin = info.funds.iter().find(|fund| fund.denom == denom);
    let sent_amount: u128 = match matching_coin {
        Some(coin) => coin.amount.into(),
        None => {
            return Err(ExecuteError::NoFundsSent {
                expected_denom: denom,
            });
        }
    };

    let current = get_bounty(deps.storage, round)?;
    let new_value = current + sent_amount;
    set_bounty(deps.storage, round, new_value);

    let mut response = Response::default();
    response.data = Some(new_value.to_be_bytes().into());
    Ok(response)
}

pub fn try_add(
    deps: DepsMut,
    info: MessageInfo,
    round: u64,
    previous_signature: Binary,
    signature: Binary,
) -> Result<Response, ExecuteError> {
    let Config {
        pubkey,
        bounty_denom,
        ..
    } = config_read(deps.storage).load()?;
    let pk = g1_from_variable(&pubkey).map_err(|_| ExecuteError::InvalidPubkey {})?;
    let valid = verify(
        &pk,
        round,
        previous_signature.as_slice(),
        signature.as_slice(),
    )
    .unwrap_or(false);

    if !valid {
        return Err(ExecuteError::InvalidSignature {});
    }

    let randomness = derive_randomness(&signature);
    beacons_storage(deps.storage).set(&round.to_be_bytes(), &randomness);

    let mut response = Response::default();
    response.data = Some(randomness.into());
    let bounty = get_bounty(deps.storage, round)?;
    if bounty != 0 {
        response.messages = vec![
            SubMsg {
                id: 0,
                gas_limit: None,
                reply_on: ReplyOn::Always,
                msg: CosmosMsg::Bank(BankMsg::Send {
                    to_address: info.sender.into_string(),
                    amount: coins(bounty, bounty_denom)
                }),
            }];
            clear_bounty(deps.storage, round);
        }
    Ok(response)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<Binary, QueryError> {
    let response = match msg {
        QueryMsg::Get { round } => to_binary(&query_get(deps, round)?)?,
        QueryMsg::Latest {} => to_binary(&query_latest(deps)?)?,
        QueryMsg::Bounties {} => to_binary(&query_bounties(deps)?)?,
    };
    Ok(response)
}

fn query_get(deps: Deps, round: u64) -> Result<GetResponse, QueryError> {
    let beacons = beacons_storage_read(deps.storage);
    let randomness = beacons.get(&round.to_be_bytes()).unwrap_or_default();
    Ok(GetResponse {
        randomness: randomness.into(),
    })
}

fn query_latest(deps: Deps) -> Result<LatestResponse, QueryError> {
    let store = beacons_storage_read(deps.storage);
    let mut iter = store.range(None, None, Order::Descending);
    let (key, value) = iter.next().ok_or(QueryError::NoBeacon {})?;

    Ok(LatestResponse {
        round: u64::from_be_bytes(Binary(key).to_array()?),
        randomness: value.into(),
    })
}

fn query_bounties(deps: Deps) -> Result<BountiesResponse, QueryError> {
    let Config { bounty_denom, .. } = config_read(deps.storage).load()?;

    let store = bounties_storage_read(deps.storage);
    let iter = store.range(None, None, Order::Ascending);

    let bounties: Result<Vec<Bounty>, _> = iter
        .map(|(key, value)| -> StdResult<Bounty> {
            let round = u64::from_be_bytes(Binary(key).to_array()?);
            let amount = coins(
                u128::from_be_bytes(Binary(value).to_array()?),
                &bounty_denom,
            );
            Ok(Bounty { round, amount })
        })
        .collect();

    Ok(BountiesResponse {
        bounties: bounties?,
    })
}

fn get_bounty(storage: &dyn Storage, round: u64) -> StdResult<u128> {
    let key = round.to_be_bytes();
    let bounties = bounties_storage_read(storage);
    let value = match bounties.get(&key) {
        Some(data) => u128::from_be_bytes(Binary(data).to_array()?),
        None => 0u128,
    };
    Ok(value)
}

fn set_bounty(storage: &mut dyn Storage, round: u64, amount: u128) {
    let key = round.to_be_bytes();
    let mut bounties = bounties_storage(storage);
    bounties.set(&key, &amount.to_be_bytes());
}

fn clear_bounty(storage: &mut dyn Storage, round: u64) {
    let key = round.to_be_bytes();
    let mut bounties = bounties_storage(storage);
    bounties.remove(&key);
}
