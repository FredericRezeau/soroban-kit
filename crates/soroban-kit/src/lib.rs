/*
    Copyright (c) 2023-2024 Frederic Kyung-jin Rezeau (오경진 吳景振)

    This file is part of soroban-kit.

    Licensed under the MIT License, this software is provided "AS IS",
    no liability assumed. For details, see the LICENSE file in the
    root directory.

    Author: Fred Kyung-jin Rezeau <fred@litemint.com>
*/

#![no_std]

//! soroban-kit official repo and documentation:
//! https://github.com/FredericRezeau/soroban-kit

pub use soroban_macros;
pub use soroban_macros::*;

pub use soroban_tools;
pub use soroban_tools::*;

// Explicit since fsm::TransitionHandler trait is feature gated.
#[cfg(feature = "state-machine")]
// Bring this trait in scope for #[derive(TransitionHandler)]
pub use soroban_tools::fsm::TransitionHandler;