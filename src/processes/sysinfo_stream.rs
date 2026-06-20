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
                upload_rate: 2_568_192,
                download: "5.12 MB/s".into(),
                download_rate: 5_369_344,
                cpu: "12.5%".into(),
                cpu_percent: 12.5,
                ram: "1.2 GB".into(),
            },
            Process {
                process: "iTerm2".into(),
                pid: "5678".into(),
                connections: "2".into(),
                upload: "128 KB/s".into(),
                upload_rate: 131_072,
                download: "256 KB/s".into(),
                download_rate: 262_144,
                cpu: "3.2%".into(),
                cpu_percent: 3.2,
                ram: "512 MB".into(),
            },
            Process {
                process: "Spotify".into(),
                pid: "9101".into(),
                connections: "4".into(),
                upload: "856 KB/s".into(),
                upload_rate: 876_544,
                download: "3.21 MB/s".into(),
                download_rate: 3_366_584,
                cpu: "8.7%".into(),
                cpu_percent: 8.7,
                ram: "890 MB".into(),
            },
            Process {
                process: "Discord".into(),
                pid: "3456".into(),
                connections: "8".into(),
                upload: "1.12 MB/s".into(),
                upload_rate: 1_173_760,
                download: "2.87 MB/s".into(),
                download_rate: 3_008_512,
                cpu: "5.1%".into(),
                cpu_percent: 5.1,
                ram: "780 MB".into(),
            },
            Process {
                process: "mdworker".into(),
                pid: "7890".into(),
                connections: "0".into(),
                upload: "0 B/s".into(),
                upload_rate: 0,
                download: "0 B/s".into(),
                download_rate: 0,
                cpu: "0.1%".into(),
                cpu_percent: 0.1,
                ram: "45 MB".into(),
            },        
        ]
    }
}
