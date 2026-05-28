use std::{io, vec};

use crossterm::{event::{self, Event, KeyCode, KeyEvent, KeyEventKind}};
use ratatui::{
    DefaultTerminal, 
    Frame, 
    buffer::Buffer, 
    layout::{Constraint, Layout, Rect}, 
    style::Stylize, 
    symbols::border, 
    text::Line, 
    widgets::{Block, Paragraph, Widget}
};


#[derive(Debug)]
pub struct App<'a>{
    exit: bool,
    posx: usize,
    posy: usize,
    map: Vec<String>,
    cells: Vec<Rect>,
    block: Block<'a>
}

impl App<'_>{

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let inner_area = self.block.inner(terminal.get_frame().area());
        let col_constraints = (self.posx..self.map[0].len() as usize).map(|_| Constraint::Length(3));
        let row_constraints = (self.posy..self.map.len()  as usize).map(|_| Constraint::Length(2));
        let horizontal = Layout::horizontal(col_constraints).spacing(-1).vertical_margin(0);
        let vertical = Layout::vertical(row_constraints).spacing(-1).vertical_margin(0);
        let rows = vertical.split(inner_area);
        self.cells = rows.iter().flat_map(|&row| horizontal.split(row).to_vec()).collect();
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    pub fn draw(&self, frame: &mut Frame) {
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
}

impl Widget for &App<'_>{
    fn render(self, area: Rect, buf: &mut Buffer) {
        for cell in self.cells.iter(){
            Paragraph::new("S".blue())
                .render(*cell, buf);
        }
       self.block.clone().render(area,buf);

    }

}

impl Default for App<'_>{
    fn default() -> App<'static>{
        App{
            exit: false,
            posx: 0,
            posy: 0,
            map: vec![
                "sssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssss".into(),
                "sssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssss".into(),
                "sssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssss".into(),
                "sssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssss".into(),
                "sssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssss".into(),
                "sssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssss".into(),
                "sssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssss".into(),
                "sssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssss".into(),
                "sssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssss".into(),
                "sssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssss".into(),],
            cells: Vec::new(),
            block: Block::bordered().title(Line::from(" NEST ".bold()).centered())
             .title_bottom(Line::from(vec![
            " Directional Movement ".into(),
            "<A/Left> ".blue().bold(),
            "<W/Right> ".blue().bold(),
            "<S/Down> ".blue().bold(),
            "<D/Up>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]).centered())
        .border_set(border::THICK)
        }
    }
}