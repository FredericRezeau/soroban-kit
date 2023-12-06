/*
    Date: 2023
    Author: Fred Kyung-jin Rezeau <fred@litemint.com>
    Copyright (c) 2023 Litemint LLC

    MIT License
*/

/// Integration tests for the soroban-macros storage module.
#[cfg(feature = "storage")]
mod tests {

    extern crate soroban_tools;
    extern crate std;

    use core::panic::AssertUnwindSafe;
    use soroban_sdk::{
        contract, contractimpl, contracttype, testutils::Address as _, Address, Env, IntoVal,
        TryFromVal, Val,
    };

    use soroban_macros::{key_constraint, storage};
    use soroban_tools::storage;

    use std::panic::catch_unwind;

    // Called from the contract instance to perform a test sequence on the storage.
    // Generic over the key (`K`) and data (`D`) contract types.
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

        // Bump the data.
        storage::bump::<K, D>(&env, &key, 1, 1);

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
    #[contracttype]
    // Key constraints are compile-time restrictions ensuring
    // that only specific key types can be used with the storage.
    #[key_constraint(AdminKeyConstraint)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum AdminKey {
        Admin,
    }

    #[contracttype]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum UserKey {
        Admin,
        User(Address),
        Session(u64),
    }

    // Contract type for admin data.
    #[contracttype]
    #[storage(Instance, AdminKeyConstraint)] // Implement the instance storage for session data.
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct AdminData {
        pub address: Address,
    }

    // Contract type for user data.
    #[contracttype]
    #[storage(Persistent)] // Implement the persistent storage for session data.
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct UserData {
        pub address: Address,
    }

    #[contract]
    pub struct TestContract;

    // Using TestContract instance to run tests.
    #[contractimpl]
    impl TestContract {
        pub fn test_instance_storage(env: Env) {
            // Run generics tests.
            run_storage_tests(
                &env,
                &AdminKey::Admin,
                AdminData {
                    address: Address::random(&env),
                },
            );
        }

        pub fn test_persistent_storage(env: Env) {
            // Run generics tests.
            run_storage_tests(
                &env,
                &UserKey::User(Address::random(&env)),
                UserData {
                    address: Address::random(&env),
                },
            );
        }

        pub fn test_temporary_storage(env: Env) {
            #[contracttype] // Contract type for session data.
            #[storage(Temporary)] // Implement the temporary storage for session data.
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct SessionData {
                pub identifier: u64,
                pub timestamp: u64,
            }

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
    fn test_macros_instance_storage() {
        let env = Env::default();
        TestContractClient::new(&env, &env.register_contract(None, TestContract))
            .test_instance_storage();
    }

    #[cfg(not(feature = "mock-storage"))]
    #[test]
    fn test_macros_persistent_storage() {
        let env = Env::default();
        TestContractClient::new(&env, &env.register_contract(None, TestContract))
            .test_persistent_storage();
    }

    #[cfg(not(feature = "mock-storage"))]
    #[test]
    fn test_macros_temporary_storage() {
        let env = Env::default();
        TestContractClient::new(&env, &env.register_contract(None, TestContract))
            .test_temporary_storage();
    }

    #[cfg(feature = "mock-storage")]
    #[test]
    fn test_macros_mock_storage() {
        #[key_constraint(TestKeyConstraint)]
        #[contracttype]
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub enum TestKey {
            Session(Address),
        }

        #[storage(Instance, TestKeyConstraint)]
        #[contracttype]
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct TestData {
            pub address: Address,
        }

        let env = Env::default();
        let key = &TestKey::Session(Address::random(&env));
        let data = TestData {
            address: Address::random(&env),
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

        // Bump the data.
        storage::bump::<TestKey, TestData>(&env, &key, 1, 1);

        // Remove the data.
        storage::remove::<TestKey, TestData>(&env, &key);
    }
}
