use sysinfo::{SystemExt, System};

pub enum TelemetryFlags {
    None,
    GeneralInfo = 1,
    Errors = 2,
    Usage = 4
}

pub fn send_telemetry(level: TelemetryFlags) -> Result<(), String> {
    let client = reqwest::blocking::Client::new();
    match level {
        TelemetryFlags::GeneralInfo => {
            let mut sys = System::new_all();
            sys.refresh_all();
            let url = format!(
                "https://api.lgs.jackz.me/mmm-telemetry.php?type=general&type={}&version={}", 
                sys.name().unwrap_or_default(), sys.os_version().unwrap_or_default()
            );
            let _ = client
                .post(url)
                .header("User-Agent", "mc-mod-manager/v1.0-alpha")
                .send();
        },    
        _ => return Err("Unsupported telemetry level".to_string())
    }
    Ok(())
}