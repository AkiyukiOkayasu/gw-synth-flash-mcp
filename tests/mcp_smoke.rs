use rmcp::{
    ServiceExt,
    transport::{ConfigureCommandExt, TokioChildProcess},
};
use std::path::PathBuf;
use tokio::process::Command;

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
async fn list_tools_smoke() {
    let exe = resolve_server_exe();

    let service = ()
        .serve(
            TokioChildProcess::new(Command::new(exe).configure(|_cmd| {
                // 引数なし
            }))
            .expect("spawn mcp server"),
        )
        .await
        .expect("connect");

    let tools = service
        .list_tools(Default::default())
        .await
        .expect("list_tools");
    let names: Vec<String> = tools
        .tools
        .into_iter()
        .map(|t| t.name.into_owned())
        .collect();

    assert!(names.iter().any(|n| n == "gowin.run_tcl"));
    assert!(names.iter().any(|n| n == "gowin.list_cables"));
    assert!(names.iter().any(|n| n == "gowin.program_fs"));

    service.cancel().await.expect("cancel");
}
