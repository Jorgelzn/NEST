use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Rect,Constraint,Layout},
    style::Stylize,
    text::Line,
    symbols::border,
    widgets::{Block, Widget,Paragraph},
    DefaultTerminal, Frame,
};


#[derive(Debug)]
pub struct App {
    exit: bool,
    posx: usize,
    posy: usize,
    zoom: usize,
    cols: usize,
    rows: usize,
}

impl App {

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        //println!("{}  {}",self.posx,self.posy);
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('a') => self.move_left(),
            KeyCode::Char('d') => self.move_right(),
            KeyCode::Char('w') => self.move_up(),
            KeyCode::Char('s') => self.move_down(),
            KeyCode::Char('z') => self.zoom_up(),
            KeyCode::Char('x') => self.zoom_down(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn move_right(&mut self) {
        self.posx += 1;
    }

    fn move_left(&mut self) {
        self.posx -= 1;
    }

    fn move_up(&mut self) {
        self.posy += 1;
    }

    fn move_down(&mut self) {
        self.posy -= 1;
    }

    fn zoom_up(&mut self) {
        self.zoom += 1;
        
    }

    fn zoom_down(&mut self) {
        self.zoom -= 1;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let col_constraints = (0..self.cols).map(|_| Constraint::Length(3));
        let row_constraints = (0..self.rows).map(|_| Constraint::Length(3));
        let horizontal = Layout::horizontal(col_constraints).spacing(1);
        let vertical = Layout::vertical(row_constraints).spacing(1);

        let rows = vertical.split(area);
        let cells = rows.iter().flat_map(|&row| horizontal.split(row).to_vec());

        for (i, cell) in cells.enumerate() {

            Paragraph::new(if i + 1 == self.posx { format!("{:02}", i + 1).blue() } else {format!("{:02}", i + 1).red()})
                .block(Block::bordered())
                .render(cell, buf);
        }


        let title = Line::from(" NEST ".bold());
        let instructions = Line::from(vec![
            " Directional Movement ".into(),
            "<A/Left> ".blue().bold(),
            "<W/Right> ".blue().bold(),
            "<S/Down> ".blue().bold(),
            "<D/Up>".blue().bold(),
            " Zoom ".into(),
            "<Z/-> ".blue().bold(),
            "<X/+> ".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        block.render(area, buf);

    }

}

impl Default for App {
    fn default() -> App {
        App {
            exit: false,
            posx: 0,
            posy: 0,
            zoom: 1,
            cols: 4,
            rows: 4
        }
    }
}