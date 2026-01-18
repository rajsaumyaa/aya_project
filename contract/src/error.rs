use thiserror::Error;

//#[derive(Error, Debug)]
pub enum ContractError {
    //#[error("Unauthorized action")]
    Unauthorized {},

    //#[error("{0}")]
    Std(#[from] cosmwasm_std::StdError),
}
