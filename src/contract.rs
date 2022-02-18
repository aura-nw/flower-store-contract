#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, FlowerInfoResponse, InstantiateMsg, QueryMsg};
use crate::state::{store, store_query, Flower};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:flower_store";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let flower = Flower {
        id: "0".to_string(),
        name: msg.name,
        amount: msg.amount,
        price: msg.price,
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let key = flower.id.as_bytes();
    store(deps.storage).save(key, &flower)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::AddNew {
            id,
            name,
            amount,
            price,
        } => add_new(deps, id, name, amount, price),
        ExecuteMsg::Sell { id, amount } => sell(deps, id, amount),
    }
}

pub fn add_new(
    deps: DepsMut,
    id: String,
    name: String,
    amount: i32,
    price: i32,
) -> Result<Response, ContractError> {
    let flower = Flower {
        id: id,
        name: name,
        amount: amount,
        price: price,
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let key = flower.id.as_bytes();
    if (store(deps.storage).may_load(key)?).is_some() {
        // id is already taken
        return Err(ContractError::IDTaken { id: flower.id });
    }
    store(deps.storage).save(key, &flower)?;
    Ok(Response::new()
        .add_attribute("method", "add_new")
        .add_attribute("id", flower.id))
}

pub fn sell(deps: DepsMut, id: String, amount: i32) -> Result<Response, ContractError> {
    let key = id.as_bytes();
    store(deps.storage).update(key, |record| {
        if let Some(mut record) = record {
            if amount > record.amount {
                return Err(ContractError::NotEnoughAmount {})
            }
            record.amount -= amount;
            Ok(record)
        } else {
            Err(ContractError::IDNotExists { id: id.clone() })
        }
    })?;

    Ok(Response::new().add_attribute("method", "sell"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetFlower { id } => query_flower(deps, id),
    }
}

fn query_flower(deps: Deps, id: String) -> StdResult<Binary> {
    let key = id.as_bytes();
    let flower = match store_query(deps.storage).may_load(key)? {
        Some(record) => Some(record),
        None => None,
    };
    let resp = FlowerInfoResponse { flower };
    to_binary(&resp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn initialization() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg {
            name: "rose".to_string(),
            amount: 10,
            price: 10,
        };
        let info = mock_info("creator", &coins(1000, "earth"));
        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
        // it worked, let's query the flower
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::GetFlower {
                id: "0".to_string(),
            },
        )
        .unwrap();
        let flower = Flower {
            id: "0".to_string(),
            name: "rose".to_string(),
            amount: 10,
            price: 10,
        };
        let expected = FlowerInfoResponse {
            flower: Some(flower),
        };
        let value: FlowerInfoResponse = from_binary(&res).unwrap();
        assert_eq!(expected, value);
    }

    #[test]
    fn works_with_add_new_and_sell() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = ExecuteMsg::AddNew {
            id: "lily_id".to_string(),
            name: "lily".to_string(),
            amount: 100,
            price: 100,
        };
        let info = mock_info("creator", &coins(1000, "earth"));
        // we can just call .unwrap() to assert this was a success
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
        // it worked, let's query the flower
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::GetFlower {
                id: "lily_id".to_string(),
            },
        )
        .unwrap();
        let flower = Flower {
            id: "lily_id".to_string(),
            name: "lily".to_string(),
            amount: 100,
            price: 100,
        };
        let expected = FlowerInfoResponse {
            flower: Some(flower),
        };
        let value: FlowerInfoResponse = from_binary(&res).unwrap();
        assert_eq!(expected, value);

        let msg = ExecuteMsg::Sell {
            id: "lily_id".to_string(),
            amount: 45,
        };
        let info = mock_info("creator", &coins(1000, "earth"));
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
        // it worked, let's query the flower
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::GetFlower {
                id: "lily_id".to_string(),
            },
        )
        .unwrap();
        let flower = Flower {
            id: "lily_id".to_string(),
            name: "lily".to_string(),
            amount: 55,
            price: 100,
        };
        let expected = FlowerInfoResponse {
            flower: Some(flower),
        };
        let value: FlowerInfoResponse = from_binary(&res).unwrap();
        assert_eq!(expected, value);
    }
}
