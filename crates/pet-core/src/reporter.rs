// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use crate::{manager::EnvManager, python_environment::PythonEnvironment};

pub trait Reporter: Send + Sync {
    fn report_manager(&self, manager: &EnvManager);
    fn report_environment(&self, env: &PythonEnvironment);
    fn report_completion(&self, duration: std::time::Duration);
}
