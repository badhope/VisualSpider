use crate::ServiceInfo;
use std::process::Command;
use encoding_rs::GBK;

fn decode_output(bytes: &[u8]) -> String {
    let (decoded, _, had_errors) = GBK.decode(bytes);
    if had_errors {
        String::from_utf8_lossy(bytes).to_string()
    } else {
        decoded.to_string()
    }
}

pub fn get_services() -> Result<Vec<ServiceInfo>, String> {
    let output = Command::new("powershell")
        .args([
            "-NoProfile",
            "-ExecutionPolicy", "Bypass",
            "-Command",
            "[Console]::OutputEncoding = [System.Text.Encoding]::UTF8; Get-Service | Select-Object Name, DisplayName, Status, StartType, CanStop, CanPauseAndContinue | ConvertTo-Json"
        ])
        .output()
        .map_err(|e| format!("Failed to get services: {}", e))?;

    let stdout = decode_output(&output.stdout);
    
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

    let full_command = format!("[Console]::OutputEncoding = [System.Text.Encoding]::UTF8; {}", command);

    let output = Command::new("powershell")
        .args([
            "-NoProfile",
            "-ExecutionPolicy", "Bypass",
            "-Command",
            &full_command
        ])
        .output()
        .map_err(|e| format!("Failed to {} service: {}", action, e))?;

    if !output.status.success() {
        return Err(decode_output(&output.stderr));
    }

    Ok(())
}
