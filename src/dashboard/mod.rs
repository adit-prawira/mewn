use std::sync::{Arc, Mutex};

use crossterm::event::KeyCode;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::widgets::Tabs;
use crate::bandwidth::resource::BandwidthStatistic;
use crate::bandwidth::user_interface::BandwidthUserInterface;
use crate::connections::user_interface::ConnectionUserInterface;
use crate::connections::resource::Connection;
use crate::packet::resource::Packet;
use crate::packet::user_interface::PacketUserInterface;
use crate::processes::resource::Process;
use crate::processes::user_interface::ProcessUserInterface;
use crate::terminal::Terminal;
use crate::theme::{GREEN, TEXT_COLOR};

pub enum Tab {
    Connections,
    Bandwidth,
    Packet,
    Process
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
    process_ui: ProcessUserInterface
}

impl Default for Dashboard {
    fn default() -> Self {
        Self { 
            current_tab: Tab::Connections,
            shared_connections: Arc::new(Mutex::new(Vec::new())),
            connection_ui: ConnectionUserInterface::default(),
            
            shared_bandwidth_statistics: Arc::new(Mutex::new(Vec::new())),
            bandwidth_ui: BandwidthUserInterface::default(),
            
            shared_packets: Arc::new(Mutex::new(Vec::new())),
            packet_ui: PacketUserInterface::default(),

            shared_processes: Arc::new(Mutex::new(Vec::new())),
            process_ui: ProcessUserInterface::default()
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
            
            self.render_tabs(f, &area);
            self.render_content(f, &area); 
        });
    }

    pub fn next_tab(&mut self) {
        self.current_tab = match self.current_tab {
            Tab::Connections => Tab::Bandwidth,
            Tab::Bandwidth => Tab::Packet,
            Tab::Packet => Tab::Process,
            Tab::Process => Tab::Connections
        }
    }


    fn render_tabs(&self, frame: &mut Frame, area: &Rect) {
        let tab_titles = vec!["Connections", "Bandwidth", "Packets", "Processes"];
        let selected = match self.current_tab {
            Tab::Connections => 0,
            Tab::Bandwidth => 1,
            Tab::Packet => 2,
            Tab::Process => 3
        };

        let tabs = Tabs::new(tab_titles)
            .select(selected)
            .style(Style::default().fg(TEXT_COLOR))
            .highlight_style(Style::default().fg(GREEN));

        let tab_area = Rect::new(area.x, area.y, area.width, 1);
        frame.render_widget(tabs, tab_area);
    }

    fn render_content(&mut self, frame: &mut Frame, area: &Rect) {
        let content_area = Rect::new(area.x, area.y + 1, area.width, area.height - 1);
    
        match self.current_tab {
            Tab::Connections => {
                let connections = self.shared_connections.lock().unwrap(); 
                self.connection_ui.render(frame, content_area, &connections)
            },
            Tab::Bandwidth => {
                let bandwidth_statistics = self.shared_bandwidth_statistics.lock().unwrap();
                self.bandwidth_ui.render(frame, content_area, &bandwidth_statistics);
            },
            Tab::Packet => {
                let packets = self.shared_packets.lock().unwrap();
                self.packet_ui.render(frame, content_area, &packets);
            },
            Tab::Process => {
                let processes = self.shared_processes.lock().unwrap();
                self.process_ui.render(frame, content_area, &processes);
            }
        };
    }

    pub fn handle_keys(&mut self, key_code: KeyCode) {
        match self.current_tab {
            Tab::Connections  => {
                match key_code {
                    KeyCode::Up => self.connection_ui.previous_row(),
                    KeyCode::Down => self.connection_ui.next_row(),
                    _ => {}
                } 
            },
            Tab::Bandwidth => {
                match key_code {
                    KeyCode::Up => self.bandwidth_ui.previous_row(),
                    KeyCode::Down => self.bandwidth_ui.next_row(),
                    _ => {}
                }
            },
            Tab::Packet => {
                match key_code {
                    KeyCode::Up => self.packet_ui.previous_row(),
                    KeyCode::Down => self.packet_ui.next_row(),
                    KeyCode::Char('t') => self.packet_ui.filter_by_tcp(),
                    KeyCode::Char('T') => self.packet_ui.filter_by_tcp(),
                    KeyCode::Char('u') => self.packet_ui.filter_by_udp(),
                    KeyCode::Char('U') => self.packet_ui.filter_by_udp(),
                    KeyCode::Char('i') => self.packet_ui.filter_by_icmp(),
                    KeyCode::Char('I') => self.packet_ui.filter_by_icmp(),
                    KeyCode::Char('a') => self.packet_ui.remove_filter(),
                    KeyCode::Char('A') => self.packet_ui.remove_filter(),
                    _ => {}
                }
            },
            Tab::Process => {
                match key_code {
                    KeyCode::Up => self.process_ui.previous_row(),
                    KeyCode::Down => self.process_ui.next_row(),
                    _ => {}
                }
            }
        };
    }
}
