//! `plc-common` 的依賴限制：只允許 `serde` 及其傳遞依賴，不得引入執行期、序列埠、HTTP 或 CLI 相關套件。

use std::collections::BTreeSet;
use std::process::Command;

const SERDE_TRANSITIVE: &[&str] = &[
    "serde",
    "serde_core",
    "serde_derive",
    "proc-macro2",
    "quote",
    "syn",
    "unicode-ident",
];

const BANNED: &[&str] = &[
    "tokio",
    "tokio-serial",
    "serialport",
    "mio-serial",
    "axum",
    "reqwest",
    "hyper",
    "clap",
];

#[test]
fn only_serde_allowed() {
    let output = Command::new(env!("CARGO"))
        .args([
            "tree",
            "-p",
            "plc-common",
            "--edges",
            "normal,build",
            "--prefix",
            "none",
            "--color",
            "never",
            "--no-default-features",
        ])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("無法執行 cargo tree");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let tree = String::from_utf8(output.stdout).expect("cargo tree 輸出不是 UTF-8");
    let names: BTreeSet<&str> = tree
        .lines()
        .filter_map(|l| l.split_whitespace().next())
        .filter(|n| *n != "plc-common")
        .collect();
    for banned in BANNED {
        assert!(!names.contains(banned), "依賴樹含有 {banned}：\n{tree}");
    }
    for name in &names {
        assert!(
            SERDE_TRANSITIVE.contains(name),
            "依賴樹含有非 serde 傳遞依賴 {name}：\n{tree}"
        );
    }
}

#[test]
fn manifest_declares_only_serde() {
    let manifest = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"))
        .expect("無法讀取 Cargo.toml");
    let mut in_deps = false;
    let mut keys = BTreeSet::new();
    for line in manifest.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            in_deps = line == "[dependencies]";
            continue;
        }
        if in_deps && !line.is_empty() && !line.starts_with('#') {
            if let Some((key, _)) = line.split_once('=') {
                keys.insert(key.trim().to_string());
            }
        }
    }
    let expected: BTreeSet<String> = ["serde".to_string()].into_iter().collect();
    assert_eq!(keys, expected, "[dependencies] 鍵集合必須恰為 {{serde}}");
}
