use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Stylize,Color},
    symbols::{border,Marker},
    text::Line,
    widgets::{Block, Widget, canvas},
    DefaultTerminal, Frame,
};


#[derive(Debug)]
pub struct App {
    exit: bool,
    posx: f64,
    posy: f64,
    zoom: f64
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
        self.posx += 1.0;
    }

    fn move_left(&mut self) {
        self.posx -= 1.0;
    }

    fn move_up(&mut self) {
        self.posy += 1.0;
    }

    fn move_down(&mut self) {
        self.posy -= 1.0;
    }

    fn zoom_up(&mut self) {
        self.zoom += 0.1;
        
    }

    fn zoom_down(&mut self) {
        self.zoom -= 0.1;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let terminal_dim = area.as_size();
        let x_bound = terminal_dim.width  as f64 * self.zoom;
        let y_bound = terminal_dim.height as f64 * self.zoom;
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

        canvas::Canvas::default()
        .x_bounds([-x_bound+self.posx, x_bound+self.posx])
        .y_bounds([-y_bound+self.posy, y_bound+self.posy])
        .marker(Marker::Braille)
        .paint(|ctx| {
                        ctx.draw(&canvas::Map {
                resolution: canvas::MapResolution::High,
                color: Color::White,
            });
        })
        .block(block)
        .render(area, buf);
    }

}

impl Default for App {
    fn default() -> App {
        App {
            exit: false,
            posx: 0.0,
            posy: 0.0,
            zoom: 1.0
        }
    }
}