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
        .x_bounds([-x_bound, x_bound])
        .y_bounds([-y_bound, y_bound])
        .marker(Marker::Braille)
        .paint(|ctx| {
            ctx.draw(&canvas::Circle {
                x: self.posx,
                y: self.posy,
                radius: 1.0,
                color: Color::Red,
            });
        }).block(block)
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

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Style;

    #[test]
    fn render() {
        let app = App::default();
        let mut buf = Buffer::empty(Rect::new(0, 0, 50, 4));

        app.render(buf.area, &mut buf);

        let mut expected = Buffer::with_lines(vec![
            "┏━━━━━━━━━━━━━ Counter App Tutorial ━━━━━━━━━━━━━┓",
            "┃                    Value: 0                    ┃",
            "┃                                                ┃",
            "┗━ Decrement <Left> Increment <Right> Quit <Q> ━━┛",
        ]);
        let title_style = Style::new().bold();
        let counter_style = Style::new().yellow();
        let key_style = Style::new().blue().bold();
        expected.set_style(Rect::new(14, 0, 22, 1), title_style);
        expected.set_style(Rect::new(28, 1, 1, 1), counter_style);
        expected.set_style(Rect::new(13, 3, 6, 1), key_style);
        expected.set_style(Rect::new(30, 3, 7, 1), key_style);
        expected.set_style(Rect::new(43, 3, 4, 1), key_style);

        assert_eq!(buf, expected);
    }

    #[test]
    fn handle_key_event() {
        let mut app = App::default();
        app.handle_key_event(KeyCode::Right.into());
        assert_eq!(app.counter, 1);

        app.handle_key_event(KeyCode::Left.into());
        assert_eq!(app.counter, 0);

        let mut app = App::default();
        app.handle_key_event(KeyCode::Char('q').into());
        assert!(app.exit);
    }
}