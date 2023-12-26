/*
    Copyright (c) 2023-2024 Frederic Kyung-jin Rezeau (오경진 吳景振)

    This file is part of soroban-kit.

    Licensed under the MIT License, this software is provided "AS IS",
    no liability assumed. For details, see the LICENSE file in the
    root directory.

    Author: Fred Kyung-jin Rezeau <fred@litemint.com>
*/

#![no_std]

#[cfg(feature = "oracle")]
pub mod oracle;

#[cfg(feature = "state-machine")]
pub mod fsm;

#[cfg(feature = "circuit-breaker")]
pub mod circuit_breaker;

#[cfg(feature = "storage")]
pub mod storage;

#[cfg(any(test, feature = "utils"))]
pub mod utils;

#[cfg(any(test, feature = "mock-storage"))]
pub mod mock_storage;
