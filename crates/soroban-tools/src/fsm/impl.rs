/*
    Date: 2023
    Author: Fred Kyung-jin Rezeau <fred@litemint.com>
    Copyright (c) 2023 Litemint LLC

    MIT License
*/

use core::marker::PhantomData;
use soroban_sdk::{contracttype, Env, IntoVal, TryFromVal, Val};

// Control the state transition process flow.
pub trait TransitionHandler<K, V> {
    // Called immediately before state validation.
    // Used to implement guard conditions for the transition (e.g., ledger sequence or time-based guards).
    fn on_guard(&self, env: &Env, state_machine: &StateMachine<K, V>)
    where
        K: Clone + IntoVal<Env, Val> + TryFromVal<Env, Val>,
        V: Clone + IntoVal<Env, Val> + TryFromVal<Env, Val>;

    // Called immediately after state validation iff validation succeeded.
    // Used to implement the effect from transitioning.
    fn on_effect(&self, env: &Env, state_machine: &StateMachine<K, V>)
    where
        K: Clone + IntoVal<Env, Val> + TryFromVal<Env, Val>,
        V: Clone + IntoVal<Env, Val> + TryFromVal<Env, Val>;
}

// Generic finite state machine using Soroban storage for state serialization.
// Support for state concurrency with regions and extended state variables to allow
// modeling of complex behaviors.
pub struct StateMachine<'a, K, V>
where
    K: 'a + IntoVal<Env, Val> + TryFromVal<Env, Val>,
    V: IntoVal<Env, Val> + TryFromVal<Env, Val>,
{
    region: &'a K,
    storage_type: StorageType,
    _data: PhantomData<*const V>,
}

impl<'a, K, V> StateMachine<'a, K, V>
where
    K: Clone + IntoVal<Env, Val> + TryFromVal<Env, Val>,
    V: Clone + IntoVal<Env, Val> + TryFromVal<Env, Val>,
{
    pub fn new(region: &'a K, storage_type: StorageType) -> Self {
        StateMachine {
            region,
            storage_type,
            _data: PhantomData,
        }
    }

    pub fn get_region(&self) -> &'a K {
        self.region
    }

    pub fn get_storage_type(&self) -> &StorageType {
        &self.storage_type
    }

    pub fn set_state(&self, env: &Env, value: V) {
        match self.storage_type {
            StorageType::Instance => env
                .storage()
                .instance()
                .set(&self.region.into_val(env), &value),
            StorageType::Persistent => env
                .storage()
                .persistent()
                .set(&self.region.into_val(env), &value),
            StorageType::Temporary => env
                .storage()
                .temporary()
                .set(&self.region.into_val(env), &value),
        }
    }

    pub fn get_state(&self, env: &Env) -> Option<V> {
        match self.storage_type {
            StorageType::Instance => env.storage().instance().get(&self.region.into_val(env)),
            StorageType::Persistent => env.storage().persistent().get(&self.region.into_val(env)),
            StorageType::Temporary => env.storage().temporary().get(&self.region.into_val(env)),
        }
    }
}

#[contracttype]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum StorageType {
    Instance,
    Persistent,
    Temporary,
}

// Default region if none is specified.
#[contracttype]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum StateMachineRegion {
    Default,
}

// Most of the code here is for pattern matching with variadic, optional parameters
// and expansion of extended state variable types and regions.
// The injected code remains minimal despite the syntax verbosity (any rust macro syntax experts?).
// See @internal arm for state validation logic.
#[macro_export]
macro_rules! impl_state_machine {
    ($instance:expr, $env:expr, $storage_type:expr, $state_enum:ident, $state_variant:ident) => {
        let state_key = $state_enum::$state_variant;
        let region_key = $crate::fsm::StateMachineRegion::Default;
        $crate::impl_state_machine!(@internal $instance, $env, $storage_type, state_key, region_key, $state_enum, $crate::fsm::StateMachineRegion);
    };
    ($instance:expr, $env:expr, $storage_type:expr, $state_enum:ident, $state_variant:ident, (),
        $region_enum:ident, $region_variant:ident, ()) => {
        let state_key = $state_enum::$state_variant;
        let region_key = $region_enum::$region_variant;
        $crate::impl_state_machine!(@internal $instance, $env, $storage_type, state_key, region_key, $state_enum, $region_enum);
    };
    ($instance:expr, $env:expr, $storage_type:expr, $state_enum:ident, $state_variant:ident,
        (), $region_enum:ident, $region_variant:ident, ($($region_tuple_value:expr),+)) => {
        let state_key = $state_enum::$state_variant;
        let region_key = $region_enum::$region_variant($($region_tuple_value),*);
        $crate::impl_state_machine!(@internal $instance, $env, $storage_type, state_key, region_key, $state_enum, $region_enum);
    };
    ($instance:expr, $env:expr, $storage_type:expr, $state_enum:ident, $state_variant:ident, ($($state_tuple_value:expr),+)) => {
        let state_key = $state_enum::$state_variant($($state_tuple_value),*);
        let region_key = $crate::fsm::StateMachineRegion::Default;
        $crate::impl_state_machine!(@internal $instance, $env, $storage_type, state_key, region_key, $state_enum, $crate::fsm::StateMachineRegion);
    };
    ($instance:expr, $env:expr, $storage_type:expr, $state_enum:ident, $state_variant:ident, ($($state_tuple_value:expr),+),
        $region_enum:ident, $region_variant:ident, ()) => {
        let state_key = $state_enum::$state_variant($($state_tuple_value),*);
        let region_key = $region_enum::$region_variant;
        $crate::impl_state_machine!(@internal $instance, $env, $storage_type, state_key, region_key, $state_enum, $region_enum);
    };
    ($instance:expr, $env:expr, $storage_type:expr, $state_enum:ident, $state_variant:ident,
        ($($state_tuple_value:expr),+),$region_enum:ident, $region_variant:ident, ($($region_tuple_value:expr),+)) => {
        let state_key = $state_enum::$state_variant($($state_tuple_value),*);
        let region_key = $region_enum::$region_variant($($region_tuple_value),*);
        $crate::impl_state_machine!(@internal $instance, $env, $storage_type, state_key, region_key, $state_enum, $region_enum);
    };
    // @internal
    (@internal $instance:expr, $env:expr, $storage_type:expr, $state_key:expr, $region_key:expr, $state_enum:ty, $region_enum:ty) => {
        let sm = $crate::fsm::StateMachine::<$region_enum, $state_enum>::new(&$region_key, $storage_type);
        $instance.on_guard($env, &sm);
        match sm.get_state(&$env) {
            Some(current_state) if current_state != $state_key =>
                panic!("Expected state {:?} but got {:?}", current_state, $state_key),
            None =>
                panic!("Expected state set in state-machine"),
            _ => {}
        }
        $instance.on_effect($env, &sm);
    };
}
