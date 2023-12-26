/*
    Copyright (c) 2023-2024 Frederic Kyung-jin Rezeau (오경진 吳景振)

    This file is part of soroban-kit.

    Licensed under the MIT License, this software is provided "AS IS",
    no liability assumed. For details, see the LICENSE file in the
    root directory.

    Author: Fred Kyung-jin Rezeau <fred@litemint.com>
*/

/// Integration tests for the soroban-tools storage module.
#[cfg(feature = "storage")]
mod tests {
    extern crate soroban_tools;
    extern crate std;

    use core::panic::AssertUnwindSafe;
    #[cfg(not(feature = "mock-storage"))]
    use soroban_sdk::{
        contract, contractimpl, contracttype, testutils::Address as _, Address, Env, IntoVal,
        TryFromVal, Val,
    };
    #[cfg(feature = "mock-storage")]
    use soroban_sdk::{contracttype, testutils::Address as _, Address, Env};

    use soroban_tools::{impl_key_constraint, impl_storage, storage};

    use std::panic::catch_unwind;

    // Called from the contract instance to perform a test sequence on the storage.
    // Generic over the key (`K`) and data (`D`) contract types.
    #[cfg(not(feature = "mock-storage"))]
    fn run_storage_tests<'a, K, D>(env: &Env, key: &'a K, data: D)
    where
        K: IntoVal<Env, Val> + TryFromVal<Env, Val> + Clone + PartialEq,
        D: IntoVal<Env, Val> + TryFromVal<Env, Val> + Clone + std::fmt::Debug + Eq + PartialEq,
        storage::StorageProxy<'a, K, D>: storage::StorageOps<D>,
    {
        // Try to get the data, should panic.
        let result = catch_unwind(AssertUnwindSafe(|| {
            assert_eq!(storage::get::<K, D>(&env, &key).unwrap(), data);
        }));
        assert!(
            result.is_err(),
            "No data found. The operation should panic."
        );

        // Try to get the data with error tolerance, should not panic.
        let result = catch_unwind(AssertUnwindSafe(|| {
            storage::get_or_else::<K, D, _, _>(&env, &key, |opt| {
                opt.unwrap_or_else(|| data.clone())
            });
        }));
        assert!(
            !result.is_err(),
            "No data found. The operation should not panic."
        );

        // Set the data.
        storage::set::<K, D>(&env, &key, &data);

        // Verify that the storage now has the data.
        assert_eq!(storage::has::<K, D>(&env, &key), true);

        // Extend data TTL.
        storage::extend_ttl::<K, D>(&env, &key, 1, 1);

        // Get the data (unwrap).
        assert_eq!(storage::get::<K, D>(&env, &key).unwrap(), data);

        // Set the data with error tolerance.
        assert_eq!(
            storage::get_or_else::<K, D, _, _>(&env, &key, |opt| opt.unwrap()),
            data
        );

        // Remove the data.
        storage::remove::<K, D>(&env, &key);

        // Verify that the data is no more available.
        assert_eq!(storage::has::<K, D>(&env, &key), false);
    }

    // Admin key.
    #[cfg(not(feature = "mock-storage"))]
    #[contracttype]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum AdminKey {
        Admin,
    }

    #[cfg(not(feature = "mock-storage"))]
    #[contracttype]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum UserKey {
        Admin,
        User(Address),
        Session(u64),
    }

    // Contract type for admin data.
    #[contracttype]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct AdminData {
        pub address: Address,
    }

    // Contract type for user data.
    #[contracttype]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct UserData {
        pub address: Address,
    }

    #[cfg(not(feature = "mock-storage"))]
    #[contract]
    pub struct TestContract;

    // Using TestContract instance to run tests.
    #[cfg(not(feature = "mock-storage"))]
    #[contractimpl]
    impl TestContract {
        pub fn test_instance_storage(env: Env) {
            // Key constraints are compile-time restrictions ensuring
            // that only specific key types can be used with the storage.
            impl_key_constraint!(AdminKey, AdminKeyConstraint);

            // Implement the Instance storage for AdminData
            // (optional) Apply the AdminKeyConstraint.
            impl_storage!(Instance, AdminData, AdminKeyConstraint);

            // Run generics tests.
            run_storage_tests(
                &env,
                &AdminKey::Admin,
                AdminData {
                    address: Address::generate(&env),
                },
            );
        }

        pub fn test_persistent_storage(env: Env) {
            // Implement the Persistent storage for UserData.
            // Since no key constraint is applied to the storage, any key
            // type can be used with it.
            impl_storage!(Persistent, UserData);

            // Run generics tests.
            run_storage_tests(
                &env,
                &UserKey::User(Address::generate(&env)),
                UserData {
                    address: Address::generate(&env),
                },
            );
        }

        pub fn test_temporary_storage(env: Env) {
            // Contract type for session data.
            #[contracttype]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct SessionData {
                pub identifier: u64,
                pub timestamp: u64,
            }

            // Implement the Temporary storage for SessionData.
            impl_storage!(Temporary, SessionData);

            // Run generics tests.
            run_storage_tests(
                &env,
                &UserKey::Session(std::u64::MAX),
                SessionData {
                    identifier: 21000000,
                    timestamp: 1225476633,
                },
            );
        }
    }

    #[cfg(not(feature = "mock-storage"))]
    #[test]
    fn test_tools_instance_storage() {
        let env = Env::default();
        TestContractClient::new(&env, &env.register_contract(None, TestContract))
            .test_instance_storage();
    }

    #[cfg(not(feature = "mock-storage"))]
    #[test]
    fn test_tools_persistent_storage() {
        let env = Env::default();
        TestContractClient::new(&env, &env.register_contract(None, TestContract))
            .test_persistent_storage();
    }

    #[cfg(not(feature = "mock-storage"))]
    #[test]
    fn test_tools_temporary_storage() {
        let env = Env::default();
        TestContractClient::new(&env, &env.register_contract(None, TestContract))
            .test_temporary_storage();
    }

    #[cfg(feature = "mock-storage")]
    #[test]
    fn test_tools_mock_storage() {
        #[contracttype]
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct TestData {
            pub address: Address,
        }

        #[contracttype]
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub enum TestKey {
            Session(Address),
        }

        // Create a constraint for the key and associate it with the storage.
        impl_key_constraint!(TestKey, TestKeyConstraint);
        impl_storage!(Instance, TestData, TestKeyConstraint);

        let env = Env::default();
        let key = &TestKey::Session(Address::generate(&env));
        let data = TestData {
            address: Address::generate(&env),
        };

        // Try to get the data, should panic.
        let result = catch_unwind(AssertUnwindSafe(|| {
            assert_eq!(storage::get::<TestKey, TestData>(&env, &key).unwrap(), data);
        }));
        assert!(
            result.is_err(),
            "No data found. The operation should panic."
        );

        // Try to get the data with error tolerance, should not panic.
        let result = catch_unwind(AssertUnwindSafe(|| {
            storage::get_or_else::<TestKey, TestData, _, _>(&env, &key, |opt| {
                opt.unwrap_or_else(|| data.clone())
            });
        }));
        assert!(
            !result.is_err(),
            "No data found. The operation should not panic."
        );

        // Set the data.
        storage::set::<TestKey, TestData>(&env, &key, &data);

        // Verify that the storage now has the data.
        assert_eq!(storage::has::<TestKey, TestData>(&env, &key), true);

        // Extend data TTL.
        storage::extend_ttl::<TestKey, TestData>(&env, &key, 1, 1);

        // Remove the data.
        storage::remove::<TestKey, TestData>(&env, &key);
    }
}
