use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, UNIX_EPOCH};

use crate::bandwidth::resource::BandwidthStatistic;
use crate::bandwidth::user_interface::BandwidthUserInterface;
use crate::connections::resource::Connection;
use crate::connections::user_interface::ConnectionUserInterface;
use crate::data_export::bandwidth_export::BandwidthExport;
use crate::data_export::connection_export::ConnectionsExport;
use crate::data_export::exporter::Exporter;
use crate::data_export::packet_export::PacketExport;
use crate::data_export::process_export::ProcessExport;
use crate::data_export::resource::ExportFormat;
use crate::molecules::help_modal::HelpModal;
use crate::packet::resource::Packet;
use crate::packet::user_interface::PacketUserInterface;
use crate::processes::resource::Process;
use crate::processes::user_interface::ProcessUserInterface;
use crate::terminal::Terminal;
use crate::theme::Theme;
use crate::utilities::serializer::Serializer;
use anyhow::{Context, Result, anyhow};
use crossterm::event::KeyCode;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::Span;
use ratatui::widgets::{Paragraph, Tabs};

pub enum Tab {
    Connections,
    Bandwidth,
    Packet,
    Process,
}

pub struct Dashboard {
    current_tab: Tab,
    shared_connections: Arc<Mutex<Vec<Connection>>>,
    connection_ui: ConnectionUserInterface,

    shared_bandwidth_statistics: Arc<Mutex<Vec<BandwidthStatistic>>>,
    bandwidth_ui: BandwidthUserInterface,

    shared_packets: Arc<Mutex<Vec<Packet>>>,
    packet_ui: PacketUserInterface,

    shared_processes: Arc<Mutex<Vec<Process>>>,
    process_ui: ProcessUserInterface,

    current_export_format: ExportFormat,
    status_message: Option<(String, Instant)>,
    help_modal_visible: bool,
}

impl Default for Dashboard {
    fn default() -> Self {
        Self {
            current_tab: Tab::Process,
            shared_connections: Arc::new(Mutex::new(Vec::new())),
            connection_ui: ConnectionUserInterface::default(),

            shared_bandwidth_statistics: Arc::new(Mutex::new(Vec::new())),
            bandwidth_ui: BandwidthUserInterface::default(),

            shared_packets: Arc::new(Mutex::new(Vec::new())),
            packet_ui: PacketUserInterface::default(),

            shared_processes: Arc::new(Mutex::new(Vec::new())),
            process_ui: ProcessUserInterface::default(),

            current_export_format: ExportFormat::Json,
            status_message: None,
            help_modal_visible: false,
        }
    }
}

impl Dashboard {
    pub fn set_shared_connections(&mut self, connections: Arc<Mutex<Vec<Connection>>>) {
        self.shared_connections = connections;
    }

    pub fn set_shared_bandwidth_statistics(&mut self, bandwidth_statistics: Arc<Mutex<Vec<BandwidthStatistic>>>) {
        self.shared_bandwidth_statistics = bandwidth_statistics;
    }

    pub fn set_shared_packets(&mut self, packets: Arc<Mutex<Vec<Packet>>>) {
        self.shared_packets = packets;
    }

    pub fn set_shared_processes(&mut self, processes: Arc<Mutex<Vec<Process>>>) {
        self.shared_processes = processes;
    }

    pub fn render(&mut self, terminal: &mut Terminal) {
        terminal.draw(|f| {
            let area = f.area();

            let (main_area, notification_area) = if self.status_message.is_some() {
                let [main_area, notification_area] = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Min(0), Constraint::Length(1)])
                    .areas::<2>(area);

                (main_area, Some(notification_area))
            } else {
                (area, None)
            };

            self.render_tabs(f, &main_area);
            self.render_content(f, &main_area);

            if let Some(notification_area) = notification_area {
                let (message, timestamp) = self.status_message.as_ref().unwrap();
                if timestamp.elapsed() > Duration::from_secs(2) {
                    self.status_message = None;
                } else {
                    let style = Style::default().fg(Theme::text_dim());
                    let text = Span::styled(message.as_str(), style);
                    let notification = Paragraph::new(text);
                    f.render_widget(notification, notification_area);
                }
            }

            if self.help_modal_visible {
                HelpModal::render(f);
            }
        });
    }

    pub fn next_tab(&mut self) {
        self.current_tab = match self.current_tab {
            Tab::Process => Tab::Bandwidth,
            Tab::Bandwidth => Tab::Connections,
            Tab::Connections => Tab::Packet,
            Tab::Packet => Tab::Process,
        }
    }

    pub fn previous_tab(&mut self) {
        self.current_tab = match self.current_tab {
            Tab::Packet => Tab::Connections,
            Tab::Connections => Tab::Bandwidth,
            Tab::Bandwidth => Tab::Process,
            Tab::Process => Tab::Packet,
        }
    }

    pub fn set_status_message(&mut self, message: String) {
        self.status_message = Some((message, Instant::now()));
    }

    pub fn toggle_help_modal(&mut self) {
        self.help_modal_visible = !self.help_modal_visible;
    }

    pub fn handle_keys(&mut self, key_code: KeyCode) {
        if self.help_modal_visible {
            if matches!(key_code, KeyCode::Esc | KeyCode::Char('?')) {
                self.help_modal_visible = false;
            }
            return;
        }
        match self.current_tab {
            Tab::Connections => self.connection_ui.handle_keys(key_code),
            Tab::Bandwidth => self.bandwidth_ui.handle_keys(key_code),
            Tab::Packet => self.packet_ui.handle_keys(key_code),
            Tab::Process => self.process_ui.handle_keys(key_code),
        };
    }

    pub fn is_capturing_keys(&mut self) -> bool {
        if self.help_modal_visible {
            return true;
        }
        match self.current_tab {
            Tab::Connections => self.connection_ui.is_searching(),
            Tab::Process => self.process_ui.is_searching(),
            Tab::Bandwidth => self.bandwidth_ui.is_searching(),
            _ => false,
        }
    }

    pub fn export_current_tab(&mut self) -> Result<()> {
        // save export into ~/.mewn-exports/
        let home = std::env::var("HOME").ok().map(std::path::PathBuf::from).unwrap_or_else(|| std::path::PathBuf::from("."));
        let directory = home.join(".mewn-exports");
        fs::create_dir_all(&directory).context("failed to create export directory")?;

        let timestamp = std::time::SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let file_extension = match self.current_export_format {
            ExportFormat::Json => "json",
            ExportFormat::Csv => "csv",
        };

        let (tab_name, count) = match self.current_tab {
            Tab::Connections => self.handle_export_connections(directory, timestamp, file_extension)?,
            Tab::Bandwidth => self.handle_export_bandwidths(directory, timestamp, file_extension)?,
            Tab::Packet => self.handle_export_packets(directory, timestamp, file_extension)?,
            Tab::Process => self.handle_export_processes(directory, timestamp, file_extension)?,
        };

        self.set_status_message(format!("Exported {} {} ({})", count, tab_name, file_extension.to_uppercase()));
        Ok(())
    }

    pub fn toggle_export_format(&mut self) {
        self.current_export_format = match self.current_export_format {
            ExportFormat::Json => ExportFormat::Csv,
            ExportFormat::Csv => ExportFormat::Json,
        };

        let label = match self.current_export_format {
            ExportFormat::Json => "JSON",
            ExportFormat::Csv => "CSV",
        };

        self.set_status_message(format!("Export format: {}", label));
    }

    fn handle_export_connections(&self, directory: PathBuf, timestamp: u64, file_extension: &str) -> Result<(&'static str, usize)> {
        let data = self
            .shared_connections
            .lock()
            .map_err(|poisoned_err| anyhow!("connections mutex poisoned: {}", poisoned_err))?;
        let path = directory.join(format!("connections_{}.{}", timestamp, file_extension));
        let content = match self.current_export_format {
            ExportFormat::Json => Serializer::json(&data),
            ExportFormat::Csv => Serializer::csv(&data, &ConnectionsExport::csv_headers(), ConnectionsExport::csv_row_fields),
        };
        fs::write(&path, content)?;
        Ok(("connections", data.len()))
    }

    fn handle_export_bandwidths(&self, directory: PathBuf, timestamp: u64, file_extension: &str) -> Result<(&'static str, usize)> {
        let data = self
            .shared_bandwidth_statistics
            .lock()
            .map_err(|poisoned_err| anyhow!("bandwidths mutex poisoned: {}", poisoned_err))?;
        let path = directory.join(format!("bandwidths_{}.{}", timestamp, file_extension));
        let content = match self.current_export_format {
            ExportFormat::Json => Serializer::json(&data),
            ExportFormat::Csv => Serializer::csv(&data, &BandwidthExport::csv_headers(), BandwidthExport::csv_row_fields),
        };

        fs::write(&path, content)?;
        Ok(("bandwidths", data.len()))
    }

    fn handle_export_packets(&self, directory: PathBuf, timestamp: u64, file_extension: &str) -> Result<(&'static str, usize)> {
        let data = self.shared_packets.lock().map_err(|poisoned_err| anyhow!("packets mutex poisoned: {}", poisoned_err))?;
        let path = directory.join(format!("packets_{}.{}", timestamp, file_extension));
        let content = match self.current_export_format {
            ExportFormat::Json => Serializer::json(&data),
            ExportFormat::Csv => Serializer::csv(&data, &PacketExport::csv_headers(), PacketExport::csv_row_fields),
        };

        fs::write(&path, content)?;
        Ok(("packets", data.len()))
    }

    fn handle_export_processes(&self, directory: PathBuf, timestamp: u64, file_extension: &str) -> Result<(&'static str, usize)> {
        let data = self.shared_processes.lock().map_err(|poisoned_err| anyhow!("processes mutex poisoned: {}", poisoned_err))?;
        let path = directory.join(format!("processes_{}.{}", timestamp, file_extension));
        let content = match self.current_export_format {
            ExportFormat::Json => Serializer::json(&data),
            ExportFormat::Csv => Serializer::csv(&data, &ProcessExport::csv_headers(), ProcessExport::csv_row_fields),
        };

        fs::write(&path, content)?;
        Ok(("processes", data.len()))
    }

    fn render_tabs(&self, frame: &mut Frame, area: &Rect) {
        let mut tab_titles: Vec<String> = vec!["Processes".into(), "Bandwidth".into(), "Connections".into(), "Packet".into()];
        let selected = match self.current_tab {
            Tab::Process => 0,
            Tab::Bandwidth => 1,
            Tab::Connections => 2,
            Tab::Packet => 3,
        };

        if self.current_tab_paused() {
            tab_titles[selected].push_str(" [PAUSED]");
        }

        let tabs = Tabs::new(tab_titles)
            .select(selected)
            .style(Style::default().fg(Theme::text()))
            .highlight_style(Style::default().fg(Theme::text_highlight()));

        let tab_area = Rect::new(area.x, area.y, area.width, 1);
        frame.render_widget(tabs, tab_area);
    }

    fn render_content(&mut self, frame: &mut Frame, area: &Rect) {
        let content_area = Rect::new(area.x, area.y + 1, area.width, area.height - 1);

        match self.current_tab {
            Tab::Connections => {
                let connections = self.shared_connections.lock().unwrap();
                self.connection_ui.render(frame, content_area, &connections)
            }
            Tab::Bandwidth => {
                let bandwidth_statistics = self.shared_bandwidth_statistics.lock().unwrap();
                self.bandwidth_ui.render(frame, content_area, &bandwidth_statistics);
            }
            Tab::Packet => {
                let packets = self.shared_packets.lock().unwrap();
                self.packet_ui.render(frame, content_area, &packets);
            }
            Tab::Process => {
                let processes = self.shared_processes.lock().unwrap();
                self.process_ui.render(frame, content_area, &processes);
            }
        };
    }

    fn current_tab_paused(&self) -> bool {
        match self.current_tab {
            Tab::Connections => self.connection_ui.is_paused(),
            Tab::Bandwidth => self.bandwidth_ui.is_paused(),
            Tab::Packet => self.packet_ui.is_paused(),
            Tab::Process => self.process_ui.is_paused(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    fn poison_mutex<T: Send + 'static>(arc: &Arc<Mutex<Vec<T>>>) {
        let arc = Arc::clone(arc);
        let handle = std::thread::spawn(move || {
            let _guard = arc.lock().unwrap();
            panic!("intentional poison");
        });
        let _ = handle.join();
    }

    fn dummy_directory() -> PathBuf {
        PathBuf::from("/tmp")
    }

    // --- poison tests ---

    #[test]
    fn given_poisoned_connections_mutex_then_export_returns_error() {
        let dashboard = Dashboard::default();
        poison_mutex(&dashboard.shared_connections);

        let result = dashboard.handle_export_connections(dummy_directory(), 0, "json");
        assert!(result.is_err());
    }

    #[test]
    fn given_poisoned_bandwidth_mutex_then_export_returns_error() {
        let dashboard = Dashboard::default();
        poison_mutex(&dashboard.shared_bandwidth_statistics);

        let result = dashboard.handle_export_bandwidths(dummy_directory(), 0, "json");
        assert!(result.is_err());
    }

    #[test]
    fn given_poisoned_packets_mutex_then_export_returns_error() {
        let dashboard = Dashboard::default();
        poison_mutex(&dashboard.shared_packets);

        let result = dashboard.handle_export_packets(dummy_directory(), 0, "json");
        assert!(result.is_err());
    }

    #[test]
    fn given_poisoned_processes_mutex_then_export_returns_error() {
        let dashboard = Dashboard::default();
        poison_mutex(&dashboard.shared_processes);

        let result = dashboard.handle_export_processes(dummy_directory(), 0, "json");
        assert!(result.is_err());
    }

    // --- happy-path tests ---

    #[test]
    fn given_healthy_connections_data_then_export_writes_json_file() {
        let dashboard = Dashboard::default();
        dashboard.shared_connections.lock().unwrap().push(Connection {
            pid: 1,
            process: "test".into(),
            local: "127.0.0.1:80".into(),
            remote: "10.0.0.1:443".into(),
            state: "ESTABLISHED".into(),
            protocol: "TCP".into(),
            country: Some("US".into()),
        });

        let tempdir = tempfile::tempdir().unwrap();
        let directory = tempdir.path().to_path_buf();
        let result = dashboard.handle_export_connections(directory.clone(), 0, "json");
        assert!(result.is_ok(), "export failed: {:?}", result.err());
        let (tab_name, count) = result.unwrap();
        assert_eq!(tab_name, "connections");
        assert_eq!(count, 1);

        let written_path = directory.join("connections_0.json");
        assert!(written_path.exists(), "file not found at {:?}", written_path);
        let content = std::fs::read_to_string(&written_path).unwrap();
        assert!(!content.is_empty(), "file is empty");
        assert!(content.contains("pid"), "content: {}", content);
        assert!(content.contains("TCP"), "content: {}", content);
    }

    #[test]
    fn given_healthy_bandwidth_data_then_export_writes_json_file() {
        let dashboard = Dashboard::default();
        dashboard.shared_bandwidth_statistics.lock().unwrap().push(BandwidthStatistic {
            name: "en0".into(),
            address: "10.0.0.1".into(),
            maximum_transmission_unit: "1500".into(),
            upload: "100B".into(),
            upload_rate: 1_000,
            download: "200B".into(),
            download_rate: 2_000,
            total: "300B".into(),
        });

        let tempdir = tempfile::tempdir().unwrap();
        let directory = tempdir.path().to_path_buf();
        let result = dashboard.handle_export_bandwidths(directory.clone(), 0, "json");
        assert!(result.is_ok(), "export failed: {:?}", result.err());
        let (tab_name, count) = result.unwrap();
        assert_eq!(tab_name, "bandwidths");
        assert_eq!(count, 1);

        let written_path = directory.join("bandwidths_0.json");
        assert!(written_path.exists(), "file not found at {:?}", written_path);
        let content = std::fs::read_to_string(&written_path).unwrap();
        assert!(!content.is_empty(), "file is empty");
        assert!(content.contains("en0"), "content: {}", content);
        assert!(content.contains("upload_rate"), "content: {}", content);
    }

    #[test]
    fn given_healthy_packets_data_then_export_writes_json_file() {
        let dashboard = Dashboard::default();
        dashboard.shared_packets.lock().unwrap().push(Packet {
            timestamp: "12:00:00".into(),
            protocol: "TCP".into(),
            source: "10.0.0.1".into(),
            destination: "10.0.0.2".into(),
            size: "100B".into(),
            source_port: 12345,
            destination_port: 443,
            raw_size: 100,
            dns_domain: None,
        });

        let tempdir = tempfile::tempdir().unwrap();
        let directory = tempdir.path().to_path_buf();
        let result = dashboard.handle_export_packets(directory.clone(), 0, "json");
        assert!(result.is_ok(), "export failed: {:?}", result.err());
        let (tab_name, count) = result.unwrap();
        assert_eq!(tab_name, "packets");
        assert_eq!(count, 1);

        let written_path = directory.join("packets_0.json");
        assert!(written_path.exists(), "file not found at {:?}", written_path);
        let content = std::fs::read_to_string(&written_path).unwrap();
        assert!(!content.is_empty(), "file is empty");
        assert!(content.contains("TCP"), "content: {}", content);
        assert!(content.contains("source_port"), "content: {}", content);
    }

    #[test]
    fn given_healthy_processes_data_then_export_writes_json_file() {
        let dashboard = Dashboard::default();
        dashboard.shared_processes.lock().unwrap().push(Process {
            process: "test".into(),
            pid: 1,
            connections: 1,
            upload: "100B".into(),
            upload_rate: 1_000,
            download: "200B".into(),
            download_rate: 2_000,
            cpu: "5%".into(),
            cpu_percent: 5.0,
            ram: "10MB".into(),
            ram_bytes: 10_000_000,
        });

        let tempdir = tempfile::tempdir().unwrap();
        let directory = tempdir.path().to_path_buf();
        let result = dashboard.handle_export_processes(directory.clone(), 0, "json");
        assert!(result.is_ok(), "export failed: {:?}", result.err());
        let (tab_name, count) = result.unwrap();
        assert_eq!(tab_name, "processes");
        assert_eq!(count, 1);

        let written_path = directory.join("processes_0.json");
        assert!(written_path.exists(), "file not found at {:?}", written_path);
        let content = std::fs::read_to_string(&written_path).unwrap();
        assert!(!content.is_empty(), "file is empty");
        assert!(content.contains("test"), "content: {}", content);
        assert!(content.contains("cpu_percent"), "content: {}", content);
    }
}
