use std::env;
use trybuild;

#[test]
fn ui() {
  // This environment variable tells build.rs that we're running trybuild tests,
  // so it won't rebuild V8.
  env::set_var("DENO_TRYBUILD", "1");

  let t = trybuild::TestCases::new();
  t.compile_fail("tests/compile_fail/boxed_local.rs");
  t.compile_fail("tests/compile_fail/handle_scope_escape_lifetime.rs");
  t.compile_fail("tests/compile_fail/handle_scope_lifetimes.rs");
  t.compile_fail("tests/compile_fail/try_catch_lifetimes.rs");

  // For unclear reasons rustc on Windows in Github Actions omits some
  // diagnostic information, causing this test to fail. It might have something
  // to do with this Rust issue: https://github.com/rust-lang/rust/issues/53081.
  if cfg!(not(windows)) || env::var("GITHUB_ACTION").is_err() {
    t.compile_fail("tests/compile_fail/handle_scope_escape_to_nowhere.rs");
  }
}
