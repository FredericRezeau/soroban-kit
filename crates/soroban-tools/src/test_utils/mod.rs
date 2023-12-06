/*
    Date: 2023
    Author: Fred Kyung-jin Rezeau <fred@litemint.com>
    Copyright (c) 2023 Litemint LLC

    MIT License
*/

/// A rudimentary mock storage allowing testing and profiling.
/// outside of Soroban environment.
/// `cargo test --features mock-storage`
#[cfg(feature = "mock-storage")]
mod mock_storage;

#[cfg(feature = "mock-storage")]
pub use mock_storage::*;
