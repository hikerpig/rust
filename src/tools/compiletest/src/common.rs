// Copyright 2012-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
pub use self::Mode::*;

use std::fmt;
use std::str::FromStr;
use std::path::PathBuf;

use test::ColorConfig;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Mode {
    CompileFail,
    ParseFail,
    RunFail,
    RunPass,
    RunPassValgrind,
    Pretty,
    DebugInfoGdb,
    DebugInfoLldb,
    Codegen,
    Rustdoc,
    CodegenUnits,
    Incremental,
    RunMake,
    Ui,
    MirOpt,
}

impl Mode {
    pub fn disambiguator(self) -> &'static str {
        // Run-pass and pretty run-pass tests could run concurrently, and if they do,
        // they need to keep their output segregated. Same is true for debuginfo tests that
        // can be run both on gdb and lldb.
        match self {
            Pretty => ".pretty",
            DebugInfoGdb => ".gdb",
            DebugInfoLldb => ".lldb",
            _ => "",
        }
    }
}

impl FromStr for Mode {
    type Err = ();
    fn from_str(s: &str) -> Result<Mode, ()> {
        match s {
            "compile-fail" => Ok(CompileFail),
            "parse-fail" => Ok(ParseFail),
            "run-fail" => Ok(RunFail),
            "run-pass" => Ok(RunPass),
            "run-pass-valgrind" => Ok(RunPassValgrind),
            "pretty" => Ok(Pretty),
            "debuginfo-lldb" => Ok(DebugInfoLldb),
            "debuginfo-gdb" => Ok(DebugInfoGdb),
            "codegen" => Ok(Codegen),
            "rustdoc" => Ok(Rustdoc),
            "codegen-units" => Ok(CodegenUnits),
            "incremental" => Ok(Incremental),
            "run-make" => Ok(RunMake),
            "ui" => Ok(Ui),
            "mir-opt" => Ok(MirOpt),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(match *self {
                              CompileFail => "compile-fail",
                              ParseFail => "parse-fail",
                              RunFail => "run-fail",
                              RunPass => "run-pass",
                              RunPassValgrind => "run-pass-valgrind",
                              Pretty => "pretty",
                              DebugInfoGdb => "debuginfo-gdb",
                              DebugInfoLldb => "debuginfo-lldb",
                              Codegen => "codegen",
                              Rustdoc => "rustdoc",
                              CodegenUnits => "codegen-units",
                              Incremental => "incremental",
                              RunMake => "run-make",
                              Ui => "ui",
                              MirOpt => "mir-opt",
                          },
                          f)
    }
}

#[derive(Clone)]
pub enum CompareMode {
    Nll
}

impl CompareMode {
    pub(crate) fn to_str(&self) -> &'static str {
        match *self {
            CompareMode::Nll => "nll"
        }
    }

    pub fn parse(s: String) -> CompareMode {
        match s.as_str() {
            "nll" => CompareMode::Nll,
            x => panic!("unknown --compare-mode option: {}", x),
        }
    }
}

#[derive(Clone)]
pub struct Config {
    /// The library paths required for running the compiler
    pub compile_lib_path: PathBuf,

    /// The library paths required for running compiled programs
    pub run_lib_path: PathBuf,

    /// The rustc executable
    pub rustc_path: PathBuf,

    /// The rustdoc executable
    pub rustdoc_path: Option<PathBuf>,

    /// The python executable to use for LLDB
    pub lldb_python: String,

    /// The python executable to use for htmldocck
    pub docck_python: String,

    /// The llvm FileCheck binary path
    pub llvm_filecheck: Option<PathBuf>,

    /// The valgrind path
    pub valgrind_path: Option<String>,

    /// Whether to fail if we can't run run-pass-valgrind tests under valgrind
    /// (or, alternatively, to silently run them like regular run-pass tests).
    pub force_valgrind: bool,

    /// The directory containing the tests to run
    pub src_base: PathBuf,

    /// The directory where programs should be built
    pub build_base: PathBuf,

    /// The name of the stage being built (stage1, etc)
    pub stage_id: String,

    /// The test mode, compile-fail, run-fail, run-pass
    pub mode: Mode,

    /// Run ignored tests
    pub run_ignored: bool,

    /// Only run tests that match this filter
    pub filter: Option<String>,

    /// Exactly match the filter, rather than a substring
    pub filter_exact: bool,

    /// Write out a parseable log of tests that were run
    pub logfile: Option<PathBuf>,

    /// A command line to prefix program execution with,
    /// for running under valgrind
    pub runtool: Option<String>,

    /// Flags to pass to the compiler when building for the host
    pub host_rustcflags: Option<String>,

    /// Flags to pass to the compiler when building for the target
    pub target_rustcflags: Option<String>,

    /// Target system to be tested
    pub target: String,

    /// Host triple for the compiler being invoked
    pub host: String,

    /// Path to / name of the GDB executable
    pub gdb: Option<String>,

    /// Version of GDB, encoded as ((major * 1000) + minor) * 1000 + patch
    pub gdb_version: Option<u32>,

    /// Whether GDB has native rust support
    pub gdb_native_rust: bool,

    /// Version of LLDB
    pub lldb_version: Option<String>,

    /// Version of LLVM
    pub llvm_version: Option<String>,

    /// Is LLVM a system LLVM
    pub system_llvm: bool,

    /// Path to the android tools
    pub android_cross_path: PathBuf,

    /// Extra parameter to run adb on arm-linux-androideabi
    pub adb_path: String,

    /// Extra parameter to run test suite on arm-linux-androideabi
    pub adb_test_dir: String,

    /// status whether android device available or not
    pub adb_device_status: bool,

    /// the path containing LLDB's Python module
    pub lldb_python_dir: Option<String>,

    /// Explain what's going on
    pub verbose: bool,

    /// Print one character per test instead of one line
    pub quiet: bool,

    /// Whether to use colors in test.
    pub color: ColorConfig,

    /// where to find the remote test client process, if we're using it
    pub remote_test_client: Option<PathBuf>,

    /// mode describing what file the actual ui output will be compared to
    pub compare_mode: Option<CompareMode>,

    // Configuration for various run-make tests frobbing things like C compilers
    // or querying about various LLVM component information.
    pub cc: String,
    pub cxx: String,
    pub cflags: String,
    pub ar: String,
    pub linker: Option<String>,
    pub llvm_components: String,
    pub llvm_cxxflags: String,
    pub nodejs: Option<String>,
}

#[derive(Clone)]
pub struct TestPaths {
    pub file: PathBuf,         // e.g., compile-test/foo/bar/baz.rs
    pub base: PathBuf,         // e.g., compile-test, auxiliary
    pub relative_dir: PathBuf, // e.g., foo/bar
}

/// Used by `ui` tests to generate things like `foo.stderr` from `foo.rs`.
pub fn expected_output_path(testpaths: &TestPaths,
                            revision: Option<&str>,
                            compare_mode: &Option<CompareMode>,
                            kind: &str) -> PathBuf {

    assert!(UI_EXTENSIONS.contains(&kind));
    let mut parts = Vec::new();

    if let Some(x) = revision { parts.push(x); }
    if let Some(ref x) = *compare_mode { parts.push(x.to_str()); }
    parts.push(kind);

    let extension = parts.join(".");
    testpaths.file.with_extension(extension)
}

pub const UI_EXTENSIONS: &[&str] = &[UI_STDERR, UI_STDOUT];
pub const UI_STDERR: &str = "stderr";
pub const UI_STDOUT: &str = "stdout";
