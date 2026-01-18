use cosmwasm_std::{
    entry_point, Addr, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use crate::error::ContractError;
use crate::msg::{CountResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

//#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let admins: Vec<Addr> = msg
        .admins
        .iter()
        .map(|a| deps.api.addr_validate(a))
        .collect::<StdResult<_>>()?;

    let state = State {
        owner: info.sender.clone(),
        admins,
        count: 0,
    };

    STATE.save(deps.storage, &state)?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

fn assert_admin(state: &State, sender: &Addr) -> Result<(), ContractError> {
    if state.owner == *sender || state.admins.contains(sender) {
        Ok(())
    } else {
        Err(ContractError::Unauthorized {})
    }
}

//#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Increment {} => increment(deps, info),
        ExecuteMsg::Decrement {} => decrement(deps, info),
        ExecuteMsg::AddAdmin { admin } => add_admin(deps, info, admin),
    }
}

fn increment(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| {
        assert_admin(&state, &info.sender)?;
        state.count += 1;
        Ok(state)
    })?;

    Ok(Response::new().add_attribute("action", "increment"))
}

fn decrement(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| {
        assert_admin(&state, &info.sender)?;
        state.count -= 1;
        Ok(state)
    })?;

    Ok(Response::new().add_attribute("action", "decrement"))
}

fn add_admin(
    deps: DepsMut,
    info: MessageInfo,
    admin: String,
) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| {
        if state.owner != info.sender {
            return Err(ContractError::Unauthorized {});
        }

        let admin_addr = deps.api.addr_validate(&admin)?;
        if !state.admins.contains(&admin_addr) {
            state.admins.push(admin_addr);
        }

        Ok(state)
    })?;

    Ok(Response::new().add_attribute("action", "add_admin"))
}

//#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<cosmwasm_std::Binary> {
    match msg {
        QueryMsg::GetCount {} => cosmwasm_std::to_binary(&query_count(deps)?),
    }
}

fn query_count(deps: Deps) -> StdResult<CountResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(CountResponse {
        count: state.count,
        owner: state.owner,
        admins: state.admins,
    })
}
