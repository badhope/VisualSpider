use crate::ProcessInfo;
use sysinfo::System;
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

pub fn get_processes() -> Result<Vec<ProcessInfo>, String> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut processes = Vec::new();

    for (pid, process) in sys.processes() {
        let cpu = process.cpu_usage();
        let memory = process.memory() * 1024;
        let name = process.name().to_string_lossy().to_string();
        let path = process.exe().map(|p| p.to_string_lossy().to_string()).unwrap_or_default();
        let user = process.user_id().map(|u| u.to_string()).unwrap_or_default();
        
        let priority = "Normal".to_string();
        
        let threads = 0u32;
        let handles = 0u32;
        
        let start_time = process.start_time();
        let start_time_str = format_timestamp(start_time);
        
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let run_time_secs = now.saturating_sub(start_time);
        let run_time_str = format_duration(run_time_secs);
        
        let command_line = process.cmd().iter()
            .map(|s| s.to_string_lossy().to_string())
            .collect::<Vec<_>>()
            .join(" ");

        processes.push(ProcessInfo {
            pid: pid.as_u32(),
            name,
            cpu,
            memory,
            path,
            user,
            priority,
            threads,
            handles,
            start_time: start_time_str,
            run_time: run_time_str,
            command_line,
        });
    }

    processes.sort_by(|a, b| b.cpu.partial_cmp(&a.cpu).unwrap_or(std::cmp::Ordering::Equal));

    Ok(processes)
}

fn format_timestamp(timestamp: u64) -> String {
    use chrono::{DateTime, Local, TimeZone};
    
    let dt: DateTime<Local> = Local.timestamp_opt(timestamp as i64, 0)
        .single()
        .unwrap_or_else(|| Local::now());
    
    dt.format("%Y-%m-%d %H:%M:%S").to_string()
}

fn format_duration(secs: u64) -> String {
    let hours = secs / 3600;
    let minutes = (secs % 3600) / 60;
    let seconds = secs % 60;
    
    if hours > 0 {
        format!("{}时{}分{}秒", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{}分{}秒", minutes, seconds)
    } else {
        format!("{}秒", seconds)
    }
}

pub fn end_process(pid: u32) -> Result<(), String> {
    let output = Command::new("taskkill")
        .args(["/F", "/PID", &pid.to_string()])
        .output()
        .map_err(|e| format!("Failed to kill process: {}", e))?;

    if !output.status.success() {
        return Err(decode_output(&output.stderr));
    }

    Ok(())
}

pub fn set_process_priority(pid: u32, priority: &str) -> Result<(), String> {
    let command = format!("[Console]::OutputEncoding = [System.Text.Encoding]::UTF8; $p = Get-Process -Id {}; $p.PriorityClass = '{}'", pid, priority);
    
    let output = Command::new("powershell")
        .args([
            "-NoProfile",
            "-ExecutionPolicy", "Bypass",
            "-Command",
            &command
        ])
        .output()
        .map_err(|e| format!("Failed to set priority: {}", e))?;

    if !output.status.success() {
        return Err(decode_output(&output.stderr));
    }

    Ok(())
}

pub fn open_file_location(path: &str) -> Result<(), String> {
    let output = Command::new("explorer")
        .args(["/select,", path])
        .spawn()
        .map_err(|e| format!("Failed to open file location: {}", e))?;

    drop(output);
    Ok(())
}
