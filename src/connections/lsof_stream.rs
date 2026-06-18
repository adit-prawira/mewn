use std::fmt::Display;
use std::process::Command;

use super::resource::Connection;

#[allow(clippy::upper_case_acronyms)]
#[derive(PartialEq, Debug)]
pub enum Protocol {
    TCP,
    UDP
}

impl Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Protocol::TCP => write!(f, "TCP"),
            Protocol::UDP => write!(f, "UDP"),
        }
    }
}

impl Protocol {
    pub fn as_str(&self) -> &str {
        match self {
            Protocol::TCP => "TCP",
            Protocol::UDP => "UDP",
        }
    }
}

/** Executes `lsof -i -n -P` and parses the output into a list of active
 *  network connections.
 *
 *  `lsof` output format (header row then data rows):
 *    COMMAND   PID   USER   FD   TYPE   DEVICE   SIZE/OFF   NODE   NAME
 *    chrome    1234  user   42u  IPv4  0x...    0t0        TCP    192.168.1.5:52532->142.250.80.46:443 (ESTABLISHED)
 *    dns-sd    567   user   8u   IPv4  0x...    0t0        UDP    *:5353
 *
 *  Columns extracted:
 *    - COMMAND → process name
 *    - PID     → process id
 *    - NAME    → protocol, local address, remote address, connection state
 *
 *  TCP entries may include a connection state in parentheses (LISTEN,
 *  ESTABLISHED, etc.). UDP entries have no state or remote address.
 *
 *  Returns an empty list if `lsof` is unavailable, returns an error
 *  exit code, or produces no parseable output.
 */
pub struct LsofStream;

impl LsofStream {
    pub fn get_connections() -> Vec<Connection>{
        // execute lsof -i -n -P 
        let output = Command::new("lsof")
            .args(["-i", "-n", "-P"])
            .output();
        let Ok(results) = output else {return Vec::new();};

        if !results.status.success() {return Vec::new();};

        let stdout = String::from_utf8_lossy(&results.stdout);
        Self::parse_lsof_output(&stdout)
    }

    fn parse_lsof_output(output: &str) -> Vec<Connection>{
        let mut connections: Vec<Connection> = Vec::new();
        let mut lines = output.lines();

        let Some(header) = lines.next() else {return Vec::new();};
        let header_parts: Vec<&str> = header.split_whitespace()
            .collect();
        let total_connection_parts = header_parts.len();
        for line in lines {
            let Some(connection) = Self::parse_line(line, total_connection_parts) else {continue;};
            connections.push(connection);
        }
        connections
    }

    fn parse_line(line: &str, expected_total_connection_parts: usize) -> Option<Connection>{
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < expected_total_connection_parts {
            return None;
        }

        // use value from COMMAND column as process name 
        let process = parts[0].to_string();

        // use value from PID column as the pid
        let pid = parts[1].to_string();
        let parsed_pid= pid.parse::<u32>().ok()?;

        // use value from NODE and NAME column and combine them
        // e.g. TCP *:52532 (LISTEN)
        let socket_info = (parts[7..]).join(" "); 
        let (protocol, local, remote, state) = Self::parse_socket_info(&socket_info)?;
        Some(Connection { 
            pid: parsed_pid, 
            process, 
            local, 
            remote, 
            state, 
            protocol 
        })
    }

    fn parse_socket_info(socket_info: &str) -> Option<(String, String, String, String)> {
        let is_tcp_protocol = socket_info.starts_with(&Protocol::TCP.to_string());
        
        if is_tcp_protocol {
            return Self::parse_tcp(socket_info);
        }

        Self::parse_udp(socket_info)    
    }

    fn parse_tcp(socket_info: &str) -> Option<(String, String, String, String)> {
        let split_socket_info: Vec<&str> = socket_info.split_whitespace().collect();
        
        let protocol = split_socket_info[0];
        if protocol != Protocol::TCP.as_str() {return None;}; 
        
        let socket_pair = split_socket_info[1];
        
        let state = if split_socket_info.len() == 3 {
            split_socket_info[2].trim_matches(|c| c == '(' || c == ')').to_string()
        } else {
            String::from("")
        };
        
        if socket_pair.contains("->") {
            let split_socket_pair: Vec<&str> = socket_pair.split("->").collect();
            let local = split_socket_pair[0].to_string();
            let remote = split_socket_pair[1].to_string();
            return Some((Protocol::TCP.to_string(), local, remote, state));
        }
        Some((Protocol::TCP.to_string(), socket_pair.to_string(), String::from(""), state))
    }

    fn parse_udp(socket_info: &str) -> Option<(String, String, String, String)> {
        let split_socket_info: Vec<&str> = socket_info.split_whitespace().collect();
        let protocol = split_socket_info[0].to_string();
        if protocol != Protocol::UDP.as_str() {return None;};

        let local = split_socket_info[1].to_string();
        
        Some((Protocol::UDP.to_string(), local, String::from(""), String::from("")))
    }
}


