use crate::DiskInfo;
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

pub fn get_disk_info() -> Result<Vec<DiskInfo>, String> {
    let output = Command::new("powershell")
        .args([
            "-NoProfile",
            "-ExecutionPolicy", "Bypass",
            "-Command",
            "[Console]::OutputEncoding = [System.Text.Encoding]::UTF8; Get-CimInstance Win32_LogicalDisk | Where-Object {$_.DriveType -eq 3} | Select-Object DeviceID, Size, FreeSpace, FileSystem, DriveType | ConvertTo-Json"
        ])
        .output()
        .map_err(|e| format!("Failed to get disk info: {}", e))?;

    let stdout = decode_output(&output.stdout);
    
    if stdout.trim().is_empty() || stdout.trim() == "null" {
        return Ok(vec![]);
    }

    let disks: Vec<serde_json::Value> = serde_json::from_str(&stdout)
        .unwrap_or_else(|_| vec![]);

    let mut result = Vec::new();
    for disk in disks {
        let total = disk["Size"].as_i64().unwrap_or(0) as u64;
        let free = disk["FreeSpace"].as_i64().unwrap_or(0) as u64;
        
        result.push(DiskInfo {
            name: disk["DeviceID"].as_str().unwrap_or("").to_string(),
            total_space: total,
            free_space: free,
            used_space: total.saturating_sub(free),
            file_system: disk["FileSystem"].as_str().unwrap_or("").to_string(),
            drive_type: "Fixed".to_string(),
        });
    }

    Ok(result)
}

pub fn cleanup_disk(drive: &str) -> Result<(), String> {
    let output = Command::new("cleanmgr")
        .args(["/d", drive])
        .spawn()
        .map_err(|e| format!("Failed to start disk cleanup: {}", e))?;

    drop(output);
    Ok(())
}

pub fn check_disk(drive: &str) -> Result<(), String> {
    let drive_letter = drive.trim_end_matches(':').trim_end_matches('\\');
    let output = Command::new("cmd")
        .args(["/C", &format!("echo y | chkdsk {}: /f", drive_letter)])
        .output()
        .map_err(|e| format!("Failed to check disk: {}", e))?;

    if !output.status.success() {
        return Err(decode_output(&output.stderr));
    }

    Ok(())
}
