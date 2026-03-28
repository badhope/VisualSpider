use crate::{CommandResult, SystemInfo};
use std::process::Command;
use std::env;
use encoding_rs::GBK;

pub fn get_system_info() -> Result<SystemInfo, String> {
    let os_name = "Windows".to_string();
    let os_version = get_os_version().unwrap_or_else(|_| "Unknown".to_string());
    let computer_name = get_computer_name();
    let user_name = get_user_name();
    let cpu = get_cpu_info().unwrap_or_else(|_| "Unknown".to_string());
    let ram = get_total_memory().unwrap_or(0);
    let architecture = env::consts::ARCH.to_string();
    let os_build = get_os_build().unwrap_or_else(|_| "".to_string());

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

fn get_computer_name() -> String {
    env::var("COMPUTERNAME").unwrap_or_else(|_| "Unknown".to_string())
}

fn get_user_name() -> String {
    env::var("USERNAME").unwrap_or_else(|_| "Unknown".to_string())
}

fn decode_output(bytes: &[u8]) -> String {
    let (decoded, _, had_errors) = GBK.decode(bytes);
    if had_errors {
        String::from_utf8_lossy(bytes).to_string()
    } else {
        decoded.to_string()
    }
}

fn get_os_version() -> Result<String, String> {
    let output = Command::new("powershell")
        .args([
            "-NoProfile",
            "-ExecutionPolicy", "Bypass",
            "-Command",
            "[Console]::OutputEncoding = [System.Text.Encoding]::UTF8; (Get-CimInstance Win32_OperatingSystem).Caption"
        ])
        .output()
        .map_err(|e| format!("Failed to get OS version: {}", e))?;

    let stdout = decode_output(&output.stdout).trim().to_string();
    if stdout.is_empty() {
        return Ok("Unknown".to_string());
    }
    Ok(stdout)
}

fn get_os_build() -> Result<String, String> {
    let output = Command::new("powershell")
        .args([
            "-NoProfile",
            "-ExecutionPolicy", "Bypass",
            "-Command",
            "[Console]::OutputEncoding = [System.Text.Encoding]::UTF8; (Get-CimInstance Win32_OperatingSystem).BuildNumber"
        ])
        .output()
        .map_err(|e| format!("Failed to get OS build: {}", e))?;

    let stdout = decode_output(&output.stdout).trim().to_string();
    Ok(stdout)
}

fn get_cpu_info() -> Result<String, String> {
    let output = Command::new("powershell")
        .args([
            "-NoProfile",
            "-ExecutionPolicy", "Bypass",
            "-Command",
            "[Console]::OutputEncoding = [System.Text.Encoding]::UTF8; (Get-CimInstance Win32_Processor).Name"
        ])
        .output()
        .map_err(|e| format!("Failed to get CPU info: {}", e))?;

    let stdout = decode_output(&output.stdout).trim().to_string();
    if stdout.is_empty() {
        return Ok("Unknown".to_string());
    }
    Ok(stdout)
}

fn get_total_memory() -> Result<u64, String> {
    let output = Command::new("powershell")
        .args([
            "-NoProfile",
            "-ExecutionPolicy", "Bypass",
            "-Command",
            "(Get-CimInstance Win32_ComputerSystem).TotalPhysicalMemory"
        ])
        .output()
        .map_err(|e| format!("Failed to get total memory: {}", e))?;

    let stdout = decode_output(&output.stdout).trim().to_string();
    let memory: u64 = stdout.parse().unwrap_or(0);
    Ok(memory)
}

pub fn execute_powershell(command: &str) -> Result<CommandResult, String> {
    let full_command = format!("[Console]::OutputEncoding = [System.Text.Encoding]::UTF8; {}", command);
    
    let output = Command::new("powershell")
        .args(["-NoProfile", "-ExecutionPolicy", "Bypass", "-Command", &full_command])
        .output()
        .map_err(|e| format!("Failed to execute PowerShell: {}", e))?;

    let stdout = decode_output(&output.stdout);
    let stderr = decode_output(&output.stderr);
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
