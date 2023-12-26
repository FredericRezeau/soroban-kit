/*
    Copyright (c) 2023-2024 Frederic Kyung-jin Rezeau (오경진 吳景振)

    This file is part of soroban-kit.

    Licensed under the MIT License, this software is provided "AS IS",
    no liability assumed. For details, see the LICENSE file in the
    root directory.

    Author: Fred Kyung-jin Rezeau <fred@litemint.com>
*/

// Note: C-style enums only, does not support reflection for
// enum variants with data.
#[macro_export]
macro_rules! reflective_enum {
    ( $name:ident { $( $variant:ident ),* $(,)? } ) => {
        #[contracttype]
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum $name {
            $($variant),+
        }

        impl $name {
            pub fn get_values(env: &Env) -> soroban_sdk::Vec<$name> {
                vec![env, $($name::$variant),+]
            }
        }
    };
}