use rmcp::{
    transport::{ConfigureCommandExt, TokioChildProcess},
    ServiceExt,
};
use serde_json::json;
use std::path::PathBuf;
use tokio::process::Command;

fn repo_root() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .parent()
        .and_then(|p| p.parent())
        .expect("CARGO_MANIFEST_DIR/tools/gowin-mcp expected")
        .to_path_buf()
}

fn resolve_server_exe() -> PathBuf {
    if let Ok(exe) = std::env::var("CARGO_BIN_EXE_gw_synth_flash_mcp") {
        return PathBuf::from(exe);
    }
    if let Ok(exe) = std::env::var("CARGO_BIN_EXE_gw-synth-flash-mcp") {
        return PathBuf::from(exe);
    }

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let exe_path = manifest_dir
        .join("target")
        .join("debug")
        .join("gw-synth-flash-mcp");
    if exe_path.exists() {
        return exe_path;
    }

    let status = std::process::Command::new("cargo")
        .args(["build", "--quiet"])
        .current_dir(&manifest_dir)
        .status()
        .expect("failed to run cargo build");
    assert!(status.success(), "cargo build failed: {status}");
    assert!(
        exe_path.exists(),
        "expected built binary at: {}",
        exe_path.display()
    );

    exe_path
}

#[tokio::test]
#[ignore]
async fn list_cables_and_program_fs_hardware() {
    let root = repo_root();
    let cwd = root.join("Firmware").join("Gowin");

    // 1) MCPサーバー起動
    let exe = resolve_server_exe();
    let service = ()
        .serve(
            TokioChildProcess::new(Command::new(exe).configure(|cmd| {
                cmd.current_dir(&cwd);
            }))
            .expect("spawn mcp server"),
        )
        .await
        .expect("connect");

    // 2) Tcl実行で .fs を生成（危険: ビルドが走ります）
    let build = service
        .call_tool(rmcp::model::CallToolRequestParams {
            meta: None,
            task: None,
            name: "gowin.run_tcl".into(),
            arguments: Some(
                json!({
                    "project_root": cwd.display().to_string(),
                    "timeout_sec": 5400,
                    "tcl_path": "run_gowin.tcl",
                    "expected_files": ["fpgaOscillator/impl/pnr/fpgaOscillator.fs"]
                })
                .as_object()
                .expect("arguments must be object")
                .clone(),
            ),
        })
        .await
        .expect("call gowin.run_tcl");

    let build_json: serde_json::Value = build.into_typed().expect("decode run_tcl result");
    eprintln!(
        "run_tcl json: {}",
        serde_json::to_string_pretty(&build_json).unwrap()
    );
    let exit_code = build_json
        .get("exit_code")
        .and_then(|v| v.as_i64())
        .unwrap_or(-1);
    assert_eq!(exit_code, 0, "run_tcl failed");

    let all_expected_ok = build_json
        .get("expected_checks")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .all(|c| c.get("exists").and_then(|e| e.as_bool()).unwrap_or(false))
        })
        .unwrap_or(false);
    assert!(all_expected_ok, "expected output files not found");

    // 3) .fs 書き込み（危険: 実機に影響します）
    let program = service
        .call_tool(rmcp::model::CallToolRequestParams {
            meta: None,
            task: None,
            name: "gowin.program_fs".into(),
            arguments: Some(
                json!({
                    "project_root": cwd.display().to_string(),
                    "timeout_sec": 180,
                    "retries": 2,
                    "device": "GW5A-25A",
                    "frequency": "15MHz",
                    "fs_file_path": "fpgaOscillator/impl/pnr/fpgaOscillator.fs"
                })
                .as_object()
                .expect("arguments must be object")
                .clone(),
            ),
        })
        .await
        .expect("call gowin.program_fs");

    let program_json: serde_json::Value = program.into_typed().expect("decode program_fs result");
    eprintln!(
        "program_fs json: {}",
        serde_json::to_string_pretty(&program_json).unwrap()
    );
    let exit_code = program_json
        .get("exit_code")
        .and_then(|v| v.as_i64())
        .unwrap_or(-1);
    assert_eq!(exit_code, 0, "program_fs failed");

    service.cancel().await.expect("cancel");
}
