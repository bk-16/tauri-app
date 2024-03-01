// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![run_octane_render])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


#[tauri::command]
fn run_octane_render(job_path: String, assets_path: String, output_path: String, output_dir: String, octane_path: String) -> Result<(), String> {
    let script_path = "/Users/bhargav/Documents/Tauri project/win-tauri-app/src/RunOctaneRender.ps1";

    // Construct the command to execute the PowerShell script
    let output = std::process::Command::new("powershell")
        .args([
            "-File", script_path,
            "-jobPath", &job_path,
            "-assetsPath", &assets_path,
            "-outputPath", &output_path,
            "-outputDir", &output_dir,
            "-octanePath", &octane_path,
        ])
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                Ok(())
            } else {
                Err(String::from_utf8_lossy(&output.stderr).to_string())
            }
        },
        Err(e) => Err(e.to_string()),
    }
}
