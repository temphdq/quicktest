/*
*  Quick Test: CLI for stress testing in competitive programming
*  Copyright (C) 2021 - Luis Miguel Báez
*  License: MIT (See the LICENSE file in the repository root directory)
*/

use std::path::PathBuf;

use super::traits::AdapterCommand;
pub struct CmpCommand {
    pub target_file: PathBuf,
    pub correct_file: PathBuf,
    pub gen_file: PathBuf,
    pub timeout: u32,
    pub memory_limit: u64,
    pub test_cases: u32,
    pub break_bad: bool,
    pub save_bad: bool,
    pub save_all: bool,
    pub run_all: bool,
    pub run_ac: bool,
    pub run_wa: bool,
    pub run_tle: bool,
    pub run_rte: bool,
    pub run_mle: bool,
    pub diff: bool,
}

#[allow(clippy::too_many_arguments)]
impl CmpCommand {
    pub fn new(
        target_file: PathBuf,
        correct_file: PathBuf,
        gen_file: PathBuf,
        timeout: u32,
        memory_limit: u64,
        test_cases: u32,
        break_bad: bool,
        save_bad: bool,
        save_all: bool,
        run_all: bool,
        run_ac: bool,
        run_wa: bool,
        run_tle: bool,
        run_rte: bool,
        run_mle: bool,
        diff: bool,
    ) -> CmpCommand {
        CmpCommand {
            target_file,
            correct_file,
            gen_file,
            timeout,
            memory_limit,
            test_cases,
            break_bad,
            save_bad,
            save_all,
            run_all,
            run_ac,
            run_wa,
            run_tle,
            run_rte,
            run_mle,
            diff,
        }
    }
}

impl AdapterCommand for CmpCommand {
    fn get_target_file(&self) -> PathBuf {
        self.target_file.clone()
    }
    fn get_correct_file(&self) -> PathBuf {
        self.correct_file.clone()
    }
    fn get_generator_file(&self) -> PathBuf {
        self.gen_file.clone()
    }
    fn get_timeout(&self) -> u32 {
        self.timeout
    }
    fn get_memory_limit(&self) -> u64 {
        self.memory_limit
    }
    fn get_test_cases(&self) -> u32 {
        self.test_cases
    }
    fn get_break_bad(&self) -> bool {
        self.break_bad
    }
    fn get_save_bad(&self) -> bool {
        self.save_bad
    }
    fn get_save_all(&self) -> bool {
        self.save_all
    }
    fn get_run_all(&self) -> bool {
        self.run_all
    }
    fn get_run_ac(&self) -> bool {
        self.run_ac
    }
    fn get_run_wa(&self) -> bool {
        self.run_wa
    }
    fn get_run_tle(&self) -> bool {
        self.run_tle
    }
    fn get_run_rte(&self) -> bool {
        self.run_rte
    }
    fn get_run_mle(&self) -> bool {
        self.run_mle
    }
    fn get_diff(&self) -> bool {
        self.diff
    }
}
