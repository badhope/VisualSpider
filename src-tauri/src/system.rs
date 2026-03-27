use crate::{CommandResult, SystemInfo};
use std::process::Command;
use std::env;

pub fn get_system_info() -> Result<SystemInfo, String> {
    let os_name = env::consts::OS.to_string();
    let os_version = get_os_version().unwrap_or_else(|_| "Unknown".to_string());
    let computer_name = hostname::get()
        .map(|h| h.to_string_lossy().to_string())
        .unwrap_or_else(|_| "Unknown".to_string());
    let user_name = whoami::username();
    let cpu = get_cpu_info().unwrap_or_else(|_| "Unknown".to_string());
    let ram = get_total_memory().unwrap_or(0);
    let architecture = env::consts::ARCH.to_string();
    let os_build = "".to_string();

    Ok(SystemInfo {
        os_name,
        os_version,
        os_build,
        computer_name,
        user_name,
        cpu,
        ram,
        architecture,
    })
}

fn get_os_version() -> Result<String, String> {
    let output = Command::new("cmd")
        .args(["/C", "wmic os get Caption /value"])
        .output()
        .map_err(|e| e.to_string())?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if line.starts_with("Caption=") {
            return Ok(line.replace("Caption=", "").trim().to_string());
        }
    }
    Ok("Unknown".to_string())
}

fn get_cpu_info() -> Result<String, String> {
    let output = Command::new("cmd")
        .args(["/C", "wmic cpu get Name /value"])
        .output()
        .map_err(|e| e.to_string())?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if line.starts_with("Name=") {
            return Ok(line.replace("Name=", "").trim().to_string());
        }
    }
    Ok("Unknown".to_string())
}

fn get_total_memory() -> Result<u64, String> {
    use sysinfo::System;
    let mut sys = System::new_all();
    sys.refresh_all();
    Ok(sys.total_memory() * 1024)
}

pub fn execute_powershell(command: &str) -> Result<CommandResult, String> {
    let output = Command::new("powershell")
        .args(["-NoProfile", "-ExecutionPolicy", "Bypass", "-Command", command])
        .output()
        .map_err(|e| format!("Failed to execute PowerShell: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let exit_code = output.status.code().unwrap_or(-1);

    Ok(CommandResult {
        success: output.status.success(),
        output: stdout,
        error: stderr,
        exit_code,
    })
}

pub fn open_system_tool(command: &str) -> Result<(), String> {
    Command::new("cmd")
        .args(["/C", "start", command])
        .spawn()
        .map_err(|e| format!("Failed to open {}: {}", command, e))?;
    Ok(())
}

pub fn get_startup_items() -> Result<Vec<serde_json::Value>, String> {
    let output = execute_powershell(
        "Get-CimInstance Win32_StartupCommand | Select-Object Name, Command, Location | ConvertTo-Json"
    )?;

    if output.output.is_empty() || output.output.trim() == "null" {
        return Ok(vec![]);
    }

    let items: Vec<serde_json::Value> = serde_json::from_str(&output.output)
        .unwrap_or_else(|_| vec![]);

    Ok(items)
}

pub fn toggle_startup_item(_name: &str, _enabled: bool) -> Result<(), String> {
    Ok(())
}

pub fn get_scheduled_tasks() -> Result<Vec<serde_json::Value>, String> {
    let output = execute_powershell(
        "Get-ScheduledTask | Where-Object {$_.State -ne 'Disabled'} | Select-Object TaskName, State, LastRunTime, NextRunTime | ConvertTo-Json"
    )?;

    if output.output.is_empty() || output.output.trim() == "null" {
        return Ok(vec![]);
    }

    let tasks: Vec<serde_json::Value> = serde_json::from_str(&output.output)
        .unwrap_or_else(|_| vec![]);

    Ok(tasks)
}

pub fn run_scheduled_task(name: &str) -> Result<(), String> {
    execute_powershell(&format!("Start-ScheduledTask -TaskName '{}'", name))?;
    Ok(())
}

pub fn disable_scheduled_task(name: &str) -> Result<(), String> {
    execute_powershell(&format!("Disable-ScheduledTask -TaskName '{}'", name))?;
    Ok(())
}

pub fn clean_temp_files() -> Result<serde_json::Value, String> {
    let output = execute_powershell(
        "$temp = $env:TEMP; $count = (Get-ChildItem $temp -Recurse -Force -ErrorAction SilentlyContinue | Measure-Object).Count; $size = (Get-ChildItem $temp -Recurse -Force -ErrorAction SilentlyContinue | Measure-Object -Property Length -Sum).Sum; Remove-Item \"$temp\\*\" -Recurse -Force -ErrorAction SilentlyContinue; @{deleted=$count; freed=$size} | ConvertTo-Json"
    )?;

    let result: serde_json::Value = serde_json::from_str(&output.output)
        .unwrap_or(serde_json::json!({"deleted": 0, "freed": 0}));

    Ok(result)
}

pub fn clean_cache_files() -> Result<serde_json::Value, String> {
    let output = execute_powershell(
        "$cache = \"$env:LOCALAPPDATA\\Microsoft\\Windows\\INetCache\"; $count = (Get-ChildItem $cache -Recurse -Force -ErrorAction SilentlyContinue | Measure-Object).Count; $size = (Get-ChildItem $cache -Recurse -Force -ErrorAction SilentlyContinue | Measure-Object -Property Length -Sum).Sum; Remove-Item \"$cache\\*\" -Recurse -Force -ErrorAction SilentlyContinue; @{deleted=$count; freed=$size} | ConvertTo-Json"
    )?;

    let result: serde_json::Value = serde_json::from_str(&output.output)
        .unwrap_or(serde_json::json!({"deleted": 0, "freed": 0}));

    Ok(result)
}

pub fn optimize_performance() -> Result<(), String> {
    execute_powershell(
        "Clear-RecycleBin -Force -ErrorAction SilentlyContinue; Remove-Item \"$env:TEMP\\*\" -Recurse -Force -ErrorAction SilentlyContinue"
    )?;
    Ok(())
}

pub fn get_env_variables() -> Result<Vec<serde_json::Value>, String> {
    let mut vars = Vec::new();
    
    for (key, value) in env::vars() {
        vars.push(serde_json::json!({
            "name": key,
            "value": value,
            "scope": "user"
        }));
    }

    Ok(vars)
}

pub fn set_env_variable(name: &str, value: &str, scope: &str) -> Result<(), String> {
    let command = if scope == "system" {
        format!("[Environment]::SetEnvironmentVariable('{}', '{}', 'Machine')", name, value)
    } else {
        format!("[Environment]::SetEnvironmentVariable('{}', '{}', 'User')", name, value)
    };

    execute_powershell(&command)?;
    Ok(())
}

pub fn delete_env_variable(name: &str, scope: &str) -> Result<(), String> {
    let command = if scope == "system" {
        format!("[Environment]::SetEnvironmentVariable('{}', $null, 'Machine')", name)
    } else {
        format!("[Environment]::SetEnvironmentVariable('{}', $null, 'User')", name)
    };

    execute_powershell(&command)?;
    Ok(())
}

pub fn get_hosts_entries() -> Result<Vec<serde_json::Value>, String> {
    let hosts_path = "C:\\Windows\\System32\\drivers\\etc\\hosts";
    let content = std::fs::read_to_string(hosts_path)
        .map_err(|e| format!("Failed to read hosts file: {}", e))?;

    let mut entries = Vec::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            entries.push(serde_json::json!({
                "ip": parts[0],
                "hostname": parts[1]
            }));
        }
    }

    Ok(entries)
}

pub fn add_hosts_entry(ip: &str, hostname: &str) -> Result<(), String> {
    let hosts_path = "C:\\Windows\\System32\\drivers\\etc\\hosts";
    let entry = format!("\n{} {}", ip, hostname);
    
    std::fs::OpenOptions::new()
        .append(true)
        .open(hosts_path)
        .and_then(|mut file| std::io::Write::write_all(&mut file, entry.as_bytes()))
        .map_err(|e| format!("Failed to write hosts file: {}", e))?;

    Ok(())
}

pub fn delete_hosts_entry(ip: &str, hostname: &str) -> Result<(), String> {
    let hosts_path = "C:\\Windows\\System32\\drivers\\etc\\hosts";
    let content = std::fs::read_to_string(hosts_path)
        .map_err(|e| format!("Failed to read hosts file: {}", e))?;

    let new_content: String = content
        .lines()
        .filter(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                return true;
            }
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            !(parts.len() >= 2 && parts[0] == ip && parts[1] == hostname)
        })
        .collect::<Vec<&str>>()
        .join("\n");

    std::fs::write(hosts_path, new_content)
        .map_err(|e| format!("Failed to write hosts file: {}", e))?;

    Ok(())
}

pub fn run_sfc_scan() -> Result<(), String> {
    execute_powershell("Start-Process sfc -ArgumentList '/scannow' -Verb RunAs -Wait")?;
    Ok(())
}

pub fn run_dism() -> Result<(), String> {
    execute_powershell("Start-Process dism -ArgumentList '/online /cleanup-image /restorehealth' -Verb RunAs -Wait")?;
    Ok(())
}

pub fn check_windows_update() -> Result<(), String> {
    open_system_tool("ms-settings:windowsupdate")?;
    Ok(())
}

mod whoami {
    pub fn username() -> String {
        std::env::var("USERNAME").unwrap_or_else(|_| "Unknown".to_string())
    }
}

mod hostname {
    pub fn get() -> Result<std::ffi::OsString, ()> {
        Ok(std::env::var("COMPUTERNAME").unwrap_or_else(|_| "Unknown".to_string()).into())
    }
}
