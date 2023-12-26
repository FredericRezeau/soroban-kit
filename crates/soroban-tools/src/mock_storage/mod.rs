/*
    Copyright (c) 2023-2024 Frederic Kyung-jin Rezeau (오경진 吳景振)

    This file is part of soroban-kit.

    Licensed under the MIT License, this software is provided "AS IS",
    no liability assumed. For details, see the LICENSE file in the
    root directory.

    Author: Fred Kyung-jin Rezeau <fred@litemint.com>
*/

/// A rudimentary mock storage allowing testing and profiling.
/// outside of Soroban environment.
/// `cargo test --features mock-storage`
#[cfg(feature = "mock-storage")]
mod mock_storage;

#[cfg(feature = "mock-storage")]
pub use mock_storage::*;
