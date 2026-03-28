use crate::NetworkConnection;
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

pub fn get_network_connections() -> Result<Vec<NetworkConnection>, String> {
    let output = Command::new("netstat")
        .args(["-ano"])
        .output()
        .map_err(|e| format!("Failed to get network connections: {}", e))?;

    let stdout = decode_output(&output.stdout);
    let mut connections = Vec::new();

    for line in stdout.lines().skip(4) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 5 {
            let protocol = parts[0].to_string();
            
            let (local_addr, local_port) = parse_address(parts[1]);
            let (remote_addr, remote_port) = parse_address(parts[2]);
            let state = parts[3].to_string();
            let pid = parts[4].parse().unwrap_or(0);

            connections.push(NetworkConnection {
                protocol,
                local_address: local_addr,
                local_port,
                remote_address: remote_addr,
                remote_port,
                state,
                pid,
                process_name: String::new(),
            });
        }
    }

    Ok(connections)
}

fn parse_address(addr: &str) -> (String, u16) {
    if addr == "*" {
        return ("*".to_string(), 0);
    }

    let parts: Vec<&str> = addr.rsplitn(2, ':').collect();
    if parts.len() == 2 {
        let port = parts[0].parse().unwrap_or(0);
        let ip = parts[1].to_string();
        (ip, port)
    } else {
        (addr.to_string(), 0)
    }
}

pub fn get_port_usage() -> Result<Vec<serde_json::Value>, String> {
    let output = Command::new("netstat")
        .args(["-ano"])
        .output()
        .map_err(|e| format!("Failed to get port usage: {}", e))?;

    let stdout = decode_output(&output.stdout);
    let mut ports = Vec::new();

    for line in stdout.lines().skip(4) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 5 {
            let (_, local_port) = parse_address(parts[1]);
            if local_port > 0 {
                let pid = parts[4].parse().unwrap_or(0);
                let protocol = parts[0].to_string();

                ports.push(serde_json::json!({
                    "port": local_port,
                    "pid": pid,
                    "protocol": protocol,
                    "processName": get_process_name(pid)
                }));
            }
        }
    }

    ports.sort_by(|a, b| a["port"].as_u64().cmp(&b["port"].as_u64()));
    ports.dedup_by(|a, b| a["port"] == b["port"]);

    Ok(ports)
}

fn get_process_name(pid: u32) -> String {
    let output = Command::new("tasklist")
        .args(["/FI", &format!("PID eq {}", pid), "/FO", "CSV", "/NH"])
        .output();

    if let Ok(output) = output {
        let stdout = decode_output(&output.stdout);
        if let Some(line) = stdout.lines().next() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() >= 1 {
                return parts[0].trim_matches('"').to_string();
            }
        }
    }

    String::new()
}

pub fn get_dns_servers() -> Result<Vec<String>, String> {
    let output = Command::new("powershell")
        .args([
            "-NoProfile",
            "-ExecutionPolicy", "Bypass",
            "-Command",
            "[Console]::OutputEncoding = [System.Text.Encoding]::UTF8; Get-DnsClientServerAddress -AddressFamily IPv4 | Select-Object -ExpandProperty ServerAddresses | Sort-Object -Unique"
        ])
        .output()
        .map_err(|e| format!("Failed to get DNS servers: {}", e))?;

    let stdout = decode_output(&output.stdout);
    let servers: Vec<String> = stdout
        .lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    Ok(servers)
}

pub fn flush_dns() -> Result<(), String> {
    let output = Command::new("ipconfig")
        .args(["/flushdns"])
        .output()
        .map_err(|e| format!("Failed to flush DNS: {}", e))?;

    if !output.status.success() {
        return Err(decode_output(&output.stderr));
    }

    Ok(())
}

pub fn release_ip() -> Result<(), String> {
    let output = Command::new("ipconfig")
        .args(["/release"])
        .output()
        .map_err(|e| format!("Failed to release IP: {}", e))?;

    if !output.status.success() {
        return Err(decode_output(&output.stderr));
    }

    Ok(())
}

pub fn renew_ip() -> Result<(), String> {
    let output = Command::new("ipconfig")
        .args(["/renew"])
        .output()
        .map_err(|e| format!("Failed to renew IP: {}", e))?;

    if !output.status.success() {
        return Err(decode_output(&output.stderr));
    }

    Ok(())
}

pub fn reset_network() -> Result<(), String> {
    let output = Command::new("netsh")
        .args(["winsock", "reset"])
        .output()
        .map_err(|e| format!("Failed to reset network: {}", e))?;

    if !output.status.success() {
        return Err(decode_output(&output.stderr));
    }

    Ok(())
}
