// Copyright 2018-2019 the Deno authors. All rights reserved. MIT license.
use cargo_gn;
use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use which::which;

fn main() {
  if cfg!(windows) {
    init_depot_tools_windows();
  }
  if !Path::new("third_party/v8/src").is_dir()
    || env::var_os("GCLIENT_SYNC").is_some()
  {
    gclient_sync();
  }

  // On windows, rustc cannot link with a V8 debug build.
  let mut gn_args = if cargo_gn::is_debug() && !cfg!(target_os = "windows") {
    vec!["is_debug=true".to_string()]
  } else {
    vec!["is_debug=false".to_string()]
  };

  if let Some(p) = env::var_os("SCCACHE") {
    cc_wrapper(&mut gn_args, &Path::new(&p));
  } else if let Ok(p) = which("sccache") {
    cc_wrapper(&mut gn_args, &p);
  } else {
    println!("cargo:warning=Not using sccache");
  }

  // gn_root needs to be an absolute path.
  let gn_root = env::current_dir()
    .unwrap()
    .into_os_string()
    .into_string()
    .unwrap();

  let gn_out = cargo_gn::maybe_gen(&gn_root, gn_args);
  assert!(gn_out.exists());
  assert!(gn_out.join("args.gn").exists());
  cargo_gn::build("rusty_v8");

  println!("cargo:rustc-link-lib=static=rusty_v8");

  if cfg!(target_os = "windows") {
    println!("cargo:rustc-link-lib=dylib=winmm");
  }
}

fn init_depot_tools_windows() {
  let depot_tools = env::current_dir()
    .unwrap()
    .join("third_party")
    .join("depot_tools");
  // Bootstrap depot_tools.
  if !depot_tools.join("git.bat").is_file() {
    let status = Command::new("cmd.exe")
      .arg("/c")
      .arg("bootstrap\\win_tools.bat")
      .current_dir(&depot_tools)
      .status()
      .expect("bootstrapping depot_tools failed");
    assert!(status.success());
  }
  // Add third_party/depot_tools and buildtools/win to PATH.
  // TODO: this should be done on all platforms.
  // TODO: buildtools/win should not be added; instead, cargo_gn should invoke
  // depot_tools/gn.bat.
  let buildtools_win =
    env::current_dir().unwrap().join("buildtools").join("win");
  // Bootstrap depot_tools.
  let path = env::var_os("PATH").unwrap();
  let paths = vec![depot_tools, buildtools_win]
    .into_iter()
    .chain(env::split_paths(&path))
    .collect::<Vec<_>>();
  let path = env::join_paths(paths).unwrap();
  env::set_var("PATH", &path);
  // TODO: cargo_gn should do this.
  env::set_var("DEPOT_TOOLS_WIN_TOOLCHAIN", "0");
}

fn git_submodule_update() {
  Command::new("git")
    .arg("submodule")
    .arg("update")
    .arg("--init")
    .status()
    .expect("git submodule update failed");
}

fn gclient_sync() {
  let root = env::current_dir().unwrap();
  let third_party = root.join("third_party");
  let gclient_rel = PathBuf::from("depot_tools/gclient.py");
  let gclient_file = third_party.join("gclient_config.py");
  assert!(gclient_file.exists());

  if !third_party.join(&gclient_rel).exists() {
    git_submodule_update();
  }

  println!("Running gclient sync to download V8. This could take a while.");

  let cmd = if cfg!(windows) { "gclient.bat" } else { "gclient" };

  let status = Command::new(cmd)
    .current_dir(&third_party)
    .arg("sync")
    .arg("--no-history")
    .arg("--shallow")
    .env("DEPOT_TOOLS_UPDATE", "0")
    .env("DEPOT_TOOLS_METRICS", "0")
    .env("GCLIENT_FILE", gclient_file)
    .status()
    .expect("gclient sync failed");
  assert!(status.success());
}

fn cc_wrapper(gn_args: &mut Vec<String>, sccache_path: &Path) {
  gn_args.push(format!("cc_wrapper={:?}", sccache_path));

  // Disable treat_warnings_as_errors until this sccache bug is fixed:
  // https://github.com/mozilla/sccache/issues/264
  if cfg!(target_os = "windows") {
    gn_args.push("treat_warnings_as_errors=false".to_string());
  }
}
