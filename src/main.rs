use std::time::{Duration, Instant};

use clap::Parser;
use crossterm::event::{self, Event, KeyCode};
use mewn::bandwidth::store::BandwidthStore;
use mewn::cat::Cat;
use mewn::cli::Cli;
use mewn::config::Config;
use mewn::connections::store::ConnectionStore;
use mewn::dashboard::Dashboard;
use mewn::packet::store::PacketStore;
use mewn::permissions::bpf::BpfAccess;
use mewn::processes::store::ProcessStore;
use mewn::terminal::Terminal;
use mewn::theme::Theme;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if Cli::parse().process().await? {
        return Ok(());
    }

    Theme::override_color(&Config::load().colors);

    if !BpfAccess::is_available() {
        eprintln!("{}", BpfAccess::help_message());
    }

    let mut terminal = Terminal::init();
    let mut cat = Cat::default();

    let connection_store = ConnectionStore::default();
    let bandwidth_store = BandwidthStore::default();
    let packet_store = PacketStore::default();
    let process_store = ProcessStore::default();

    let shared_connections = connection_store.watch().await;
    let shared_bandwidth_statistics = bandwidth_store.watch().await;
    let shared_packets = packet_store.watch().await;
    let shared_process = process_store.watch(shared_connections.clone(), shared_packets.clone()).await;

    let mut dashboard = Dashboard::default();

    dashboard.set_shared_connections(shared_connections);
    dashboard.set_shared_bandwidth_statistics(shared_bandwidth_statistics);
    dashboard.set_shared_packets(shared_packets);
    dashboard.set_shared_processes(shared_process);

    loop {
        cat.animate(&mut terminal);
        if event::poll(Duration::from_millis(50)).expect("poll failed")
            && let Ok(Event::Key(_)) = event::read()
        {
            break;
        }

        if cat.is_complete() {
            break;
        }
    }

    terminal.clear_screen().expect("failed to clear screen");

    loop {
        let frame_start = Instant::now();
        dashboard.render(&mut terminal);
        let frame_ms = frame_start.elapsed().as_millis();
        if frame_ms > 100 {
            eprintln!("WARN: frame took {}ms", frame_ms);
        }

        if event::poll(Duration::from_millis(50)).expect("poll failed")
            && let Ok(Event::Key(key)) = event::read()
        {
            match key.code {
                KeyCode::Char('q') | KeyCode::Char('Q') if !dashboard.is_capturing_keys() => return Ok(()),
                KeyCode::Tab => dashboard.next_tab(),
                KeyCode::BackTab => dashboard.previous_tab(),
                _ => {
                    dashboard.handle_keys(key.code);
                }
            }
        }
    }
}
