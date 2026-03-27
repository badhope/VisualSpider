use crate::ProcessInfo;
use sysinfo::{System, ProcessesToUpdate, ProcessStatus};
use std::process::Command;

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
        let priority = process.priority();

        processes.push(ProcessInfo {
            pid: pid.as_u32(),
            name,
            cpu,
            memory,
            path,
            user,
            priority,
        });
    }

    processes.sort_by(|a, b| b.cpu.partial_cmp(&a.cpu).unwrap_or(std::cmp::Ordering::Equal));

    Ok(processes)
}

pub fn end_process(pid: u32) -> Result<(), String> {
    let output = Command::new("taskkill")
        .args(["/F", "/PID", &pid.to_string()])
        .output()
        .map_err(|e| format!("Failed to kill process: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    Ok(())
}
