use super::resource::Process;


pub struct SysinfoStream; 

impl SysinfoStream {
    pub fn get_processes() -> Vec<Process> {
        Self::mock_data()
    } 

    fn mock_data() -> Vec<Process> {
        vec![
             Process {
                process: "Google Chrome".into(),
                pid: "1234".into(),
                connections: "12".into(),
                upload: "2.45 MB/s".into(),
                download: "5.12 MB/s".into(),
            },
            Process {
                process: "iTerm2".into(),
                pid: "5678".into(),
                connections: "2".into(),
                upload: "128 KB/s".into(),
                download: "256 KB/s".into(),
            },
            Process {
                process: "Spotify".into(),
                pid: "9101".into(),
                connections: "4".into(),
                upload: "856 KB/s".into(),
                download: "3.21 MB/s".into(),
            },
            Process {
                process: "Discord".into(),
                pid: "3456".into(),
                connections: "8".into(),
                upload: "1.12 MB/s".into(),
                download: "2.87 MB/s".into(),
            },
            Process {
                process: "mdworker".into(),
                pid: "7890".into(),
                connections: "0".into(),
                upload: "0 B/s".into(),
                download: "0 B/s".into(),
            },        
        ]
    }
}
