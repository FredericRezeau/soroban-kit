/*
    Date: 2023
    Author: Fred Kyung-jin Rezeau <fred@litemint.com>
    Copyright (c) 2023 Litemint LLC

    MIT License
*/

use soroban_sdk::contracttype;

#[macro_export]
macro_rules! impl_circuit_breaker_state_machine {
    ($instance:expr, $env:expr, $trigger:expr, $storage_type:expr, $state_enum:ident, $state_variant:ident) => {
        let state_key = $state_variant;
        let region_key = $crate::circuit_breaker::Circuit::Default;
        $crate::impl_circuit_breaker_state_machine!(@internal $instance, $env, $trigger, $storage_type, state_key,
            region_key, $state_enum, $crate::circuit_breaker::Circuit);
    };
    ($instance:expr, $env:expr, $trigger:expr, $storage_type:expr, $state_enum:ident, $state_variant:ident,
        $region_enum:ident, $region_variant:ident, ()) => {
        let state_key = $state_variant;
        let region_key = $region_enum::$region_variant;
        $crate::impl_circuit_breaker_state_machine!(@internal $instance, $env, $trigger, $storage_type, state_key,
            region_key, $state_enum, $region_enum);
    };
    ($instance:expr, $env:expr, $trigger:expr, $storage_type:expr, $state_enum:ident, $state_variant:ident,
        $region_enum:ident, $region_variant:ident, ($($region_tuple_value:expr),+)) => {
        let state_key = $state_variant;
        let region_key = $region_enum::$region_variant($($region_tuple_value),*);
        $crate::impl_circuit_breaker_state_machine!(@internal $instance, $env, $trigger, $storage_type, state_key,
            region_key, $state_enum, $region_enum);
    };
    // @internal
    (@internal $instance:expr, $env:expr, $trigger:expr, $storage_type:expr, $state_key:expr,
        $region_key:expr, $state_enum:ty, $region_enum:ty) => {
            let sm = $crate::fsm::StateMachine::<$region_enum, $state_enum>::new(&$region_key, $storage_type);
            if $trigger {
                sm.set_state($env, &$state_key);
            }
            else {
                if sm.get_state($env).is_none() {
                    sm.set_state($env, &false); // Default circuit state is closed (false).
                }
                $instance.on_guard($env, &sm);
                assert_eq!(sm.get_state(&$env).unwrap(), $state_key);
                $instance.on_effect($env, &sm);
            }
    };
}

#[contracttype]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Circuit {
    Default,
}