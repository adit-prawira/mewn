use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Paragraph, Tabs};

use crate::connections::Connection;
use crate::terminal::Terminal;

pub enum Tab {
    Connections,
    Bandwidth
}

pub struct Dashboard {
    current_tab: Tab
}

impl Default for Dashboard {
    fn default() -> Self {
        Self { current_tab: Tab::Connections }
    }
}

impl Dashboard {
    pub fn render(&self, terminal: &mut Terminal) {
        terminal.draw(|f| {
            let area = f.area();
            
            self.render_tabs(f, &area);
            self.render_content(f, &area); 
        });
    }

    pub fn next_tab(&mut self) {
        self.current_tab = match self.current_tab {
            Tab::Bandwidth => Tab::Connections,
            Tab::Connections => Tab::Bandwidth
        }
    }


    fn render_tabs(&self, frame: &mut Frame, area: &Rect) {
        let tab_titles = vec!["Connections", "Bandwidth"];
        let selected = match self.current_tab {
            Tab::Connections => 0,
            Tab::Bandwidth => 1
        };

        let tabs = Tabs::new(tab_titles)
            .select(selected)
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Yellow));

        let tab_area = Rect::new(area.x, area.y, area.width, 1);
        frame.render_widget(tabs, tab_area);
    }

    fn render_content(&self, frame: &mut Frame, area: &Rect) {
        let content_area = Rect::new(area.x, area.y + 1, area.width, area.height - 1);
       
        match self.current_tab {
            Tab::Connections => Connection::render(frame, content_area),
            Tab::Bandwidth => {
                let text = "Bandwidth Tab - Coming Soon";
                let line = Line::from(text).style(Style::default().fg(Color::White));
                let paragraph = Paragraph::new(line);

                frame.render_widget(paragraph, content_area); 
            } 
        };
    }
}
