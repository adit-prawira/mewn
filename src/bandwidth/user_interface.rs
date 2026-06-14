use std::collections::HashMap;
use std::time::{Duration, Instant};

use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Cell, Padding, Paragraph, Row, Sparkline, Table};

use crate::bandwidth::netstat_stream::NetstatStream;
use crate::theme::{GREEN, PRIMARY, TEXT_COLOR, TEXT_COLOR_DARKER, YELLOW};

use super::resource::BandwidthStatistic;

#[derive(Default)]
pub struct BandwidthUserInterface {
    selected_row: usize,
    scroll_offset: usize,
    upload_history: HashMap<String, Vec<u64>>,
    download_history: HashMap<String, Vec<u64>>,
    last_push_at: Option<Instant> 
}

impl BandwidthUserInterface {
    pub fn render(&mut self, frame: &mut Frame, area: Rect, bandwidth_statistics: &[BandwidthStatistic]) {
        self.selected_row = self.selected_row.min(bandwidth_statistics.len().saturating_sub(1));
        
        let [table_area, graph_area] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Fill(1), Constraint::Percentage(40)])
            .areas::<2>(area);

        let viewport = (table_area.height as usize).saturating_sub(3).max(1);

        let should_push = self.last_push_at.is_none_or(|time| time.elapsed() >= Duration::from_secs(1));
        
        if should_push {
            // updating ring buffer and track 60 seconds worth of history data 
            for statistics in bandwidth_statistics { 
                let download = self.download_history.entry(statistics.name.to_string()).or_default();
                download.truncate(59);
                download.push(statistics.download_rate);

                let upload = self.upload_history.entry(statistics.name.to_string()).or_default();
                upload.truncate(59);
                upload.push(statistics.upload_rate);
            }
            self.last_push_at = Some(Instant::now());
        }

        if self.selected_row < self.scroll_offset {
            self.scroll_offset = self.selected_row;
        }

        if self.selected_row >= self.scroll_offset + viewport {
            self.scroll_offset = self.selected_row.saturating_sub(viewport.saturating_sub(1));
        }

        let header_cells = ["", "", "Name", "Address", "Upload", "Download", "Total", "Maximum Transmission Unit", ""]
            .iter() 
            .map(|header| {
                let style = Style::default().fg(TEXT_COLOR).bold();
                Cell::from(*header).style(style)
            });
        let default_text_style = Style::default().fg(TEXT_COLOR_DARKER);
        let table_header = Row::new(header_cells).height(1);
        let table_rows = bandwidth_statistics.iter().enumerate()
            .skip(self.scroll_offset)
            .take(viewport)
            .map(|(index, bandwidth_statistic)| {
                let is_selected = index == self.selected_row;
                let selected_indicator = if is_selected {"▶".to_string()} else {String::from("")};
                let style = if is_selected {
                    Style::default()
                        .fg(Color::Gray)
                        .bg(Color::Rgb(132, 75, 92))
                } else {
                    Style::default()
                        .fg(Color::Gray)
                };

                Row::new([
                    Cell::from(""),
                    Cell::from(selected_indicator).style(default_text_style),
                    Cell::from(bandwidth_statistic.name.to_string()).style(default_text_style),
                    Cell::from(bandwidth_statistic.address.to_string()).style(default_text_style),
                    Cell::from(bandwidth_statistic.upload.to_string()).style(Style::default().fg(GREEN)),
                    Cell::from(bandwidth_statistic.download.to_string()).style(Style::default().fg(YELLOW)),
                    Cell::from(bandwidth_statistic.total.to_string()).style(PRIMARY), 
                    Cell::from(
                       Line::from(Span::raw(&bandwidth_statistic.maximum_transmission_unit))
                        .alignment(Alignment::Right)
                    ).style(default_text_style),
                    Cell::from("")
                ]).style(style)
            });
        let content_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(PRIMARY))
            .padding(Padding::new(2, 2, 0, 0));

        let table = Table::new(table_rows, [
            Constraint::Length(1),
            Constraint::Length(2),
            Constraint::Length(10),
            Constraint::Length(20),
            Constraint::Length(15),
            Constraint::Length(15),
            Constraint::Length(15), 
            Constraint::Length(25),
            Constraint::Length(1)
        ])
        .header(table_header)
        .block(content_block);

        frame.render_widget(table, table_area);

        let Some(selected_statistic) = bandwidth_statistics.get(self.selected_row) else {return;};

        let upload_data = self.upload_history.get(&selected_statistic.name)
            .map(|datum| datum.as_slice())
            .unwrap_or(&[]);
        let download_data = self.download_history.get(&selected_statistic.name)
            .map(|datum| datum.as_slice())
            .unwrap_or(&[]);

        let [upload_area, download_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Fill(1), Constraint::Fill(1)])
            .areas::<2>(graph_area);
        
        let upload_max = upload_data.iter().max().copied().unwrap_or(0);
        let download_max = download_data.iter().max().copied().unwrap_or(0);

        let [upload_graph_area, upload_x_axis] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Fill(1), Constraint::Length(1)])
            .areas::<2>(upload_area);
        let upload_graph = Sparkline::default()
            .block(Block::default()
               .title(format!("Upload Rate ({}) [max: {}]", selected_statistic.upload, NetstatStream::format_bytes_per_seconds(upload_max as f64)))
               .title_style(Style::default().fg(TEXT_COLOR))
               .borders(Borders::ALL)
               .border_type(BorderType::Rounded)
               .style(Style::default().fg(PRIMARY))
            )
            .data(upload_data)
            .max(upload_max)
            .style(Style::default().fg(GREEN));
        frame.render_widget(upload_graph, upload_graph_area);
        frame.render_widget(
            Paragraph::new({
                let left = "60s ";
                let right = " now";
                let width = upload_x_axis.width as usize;
                let fill = if width > left.len() + right.len() {
                  "─".repeat(width - left.len() - right.len())
                } else {
                  String::new()
                };
                Line::from(Span::styled(format!("{left}{fill}{right}"), default_text_style))
            }),
            upload_x_axis,
        );        

        let [download_graph_area, download_x_axis] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Fill(1), Constraint::Length(1)])
            .areas::<2>(download_area);
        let download_graph = Sparkline::default()
            .block(Block::default()
               .title(format!("Download Rate ({}) [max: {}]", selected_statistic.download, NetstatStream::format_bytes_per_seconds(download_max as f64)))
               .title_style(Style::default().fg(TEXT_COLOR))
               .borders(Borders::ALL)
               .border_type(BorderType::Rounded)
               .style(Style::default().fg(PRIMARY))
            )
            .data(download_data)
            .max(download_max)
            .style(Style::default().fg(YELLOW));

        frame.render_widget(download_graph, download_graph_area);
        frame.render_widget(
            Paragraph::new({
                let left = "60s ";
                let right = " now";
                let width = download_x_axis.width as usize;
                let fill = if width > left.len() + right.len() {
                  "─".repeat(width - left.len() - right.len())
                } else {
                  String::new()
                };
                Line::from(Span::styled(format!("{left}{fill}{right}"), default_text_style))
            }),
            download_x_axis,
        );        

    }

    pub fn next_row(&mut self) {
        self.selected_row = self.selected_row.saturating_add(1);
    }

    pub fn previous_row(&mut self) {
        self.selected_row = self.selected_row.saturating_sub(1);
    }
}
