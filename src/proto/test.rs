use std::path::PathBuf;

use monostate::MustBe;
use rustc_hash::FxHashMap;
use serde::Deserialize;

use crate::proto::Snapshot;

#[derive(Deserialize, Debug)]
pub struct TestOutput {
    pub dir: PathBuf,
    pub results: FxHashMap<String, TestResult>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum TestResult {
    Success(MustBe!(true)),
    Failure {
        #[serde(flatten)]
        snapshot: Snapshot,
        old: bool,
    },
}
