// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use std::{collections::HashMap, path::PathBuf};

use pet_core::os_environment::Environment;

#[allow(dead_code)]
pub fn resolve_test_path(paths: &[&str]) -> PathBuf {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests");

    paths.iter().for_each(|p| root.push(p));

    root
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct TestEnvironment {
    vars: HashMap<String, String>,
    home: Option<PathBuf>,
    root: Option<PathBuf>,
    globals_locations: Vec<PathBuf>,
}
#[allow(dead_code)]
pub fn create_test_environment(
    root: Option<PathBuf>,
    home: Option<PathBuf>,
    vars: HashMap<String, String>,
    globals_locations: Vec<PathBuf>,
) -> TestEnvironment {
    impl Environment for TestEnvironment {
        fn get_env_var(&self, key: String) -> Option<String> {
            self.vars.get(&key).cloned()
        }
        fn get_root(&self) -> Option<PathBuf> {
            self.root.clone()
        }
        fn get_user_home(&self) -> Option<PathBuf> {
            self.home.clone()
        }
        fn get_know_global_search_locations(&self) -> Vec<PathBuf> {
            self.globals_locations.clone()
        }
    }
    TestEnvironment {
        vars,
        home,
        root,
        globals_locations,
    }
}
