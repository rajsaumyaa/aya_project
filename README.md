# Counter With Roles

## Contract Summary
A beginner-friendly Rust smart contract demonstrating role-based
access control over on-chain state.

## How to Build & Test
This is a learning-focused, code-only submission following CosmWasm
smart contract conventions. Local deployment was not required.

## State & Flows
Owner initializes the contract and assigns admins.
Admins can increment or decrement the counter.
Anyone can query the current value.

## Known Limitations
- Admin removal not supported
- No rate limiting
- No governance or voting
