use crate::ServiceInfo;
use std::process::Command;

pub fn get_services() -> Result<Vec<ServiceInfo>, String> {
    let output = Command::new("powershell")
        .args([
            "-NoProfile",
            "-ExecutionPolicy", "Bypass",
            "-Command",
            "Get-Service | Select-Object Name, DisplayName, Status, StartType, CanStop, CanPauseAndContinue | ConvertTo-Json"
        ])
        .output()
        .map_err(|e| format!("Failed to get services: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    if stdout.trim().is_empty() || stdout.trim() == "null" {
        return Ok(vec![]);
    }

    let services: Vec<serde_json::Value> = serde_json::from_str(&stdout)
        .unwrap_or_else(|_| vec![]);

    let mut result = Vec::new();
    for svc in services {
        result.push(ServiceInfo {
            name: svc["Name"].as_str().unwrap_or("").to_string(),
            display_name: svc["DisplayName"].as_str().unwrap_or("").to_string(),
            status: svc["Status"].as_str().unwrap_or("Unknown").to_string(),
            start_type: format_start_type(&svc["StartType"]),
            can_stop: svc["CanStop"].as_bool().unwrap_or(false),
            can_pause: svc["CanPauseAndContinue"].as_bool().unwrap_or(false),
        });
    }

    Ok(result)
}

fn format_start_type(value: &serde_json::Value) -> String {
    match value.as_i64() {
        Some(2) => "Automatic".to_string(),
        Some(3) => "Manual".to_string(),
        Some(4) => "Disabled".to_string(),
        _ => value.as_str().unwrap_or("Unknown").to_string(),
    }
}

pub fn control_service(name: &str, action: &str) -> Result<(), String> {
    let command = match action {
        "start" => format!("Start-Service -Name '{}'", name),
        "stop" => format!("Stop-Service -Name '{}' -Force", name),
        "restart" => format!("Restart-Service -Name '{}' -Force", name),
        _ => return Err(format!("Unknown action: {}", action)),
    };

    let output = Command::new("powershell")
        .args([
            "-NoProfile",
            "-ExecutionPolicy", "Bypass",
            "-Command",
            &command
        ])
        .output()
        .map_err(|e| format!("Failed to {} service: {}", action, e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    Ok(())
}
