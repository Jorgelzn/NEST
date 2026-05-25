use ratatui::style::{Stylize,Color};
use ratatui::widgets::{Block, Paragraph};
use ratatui::widgets::canvas::{Canvas, Line, Map, MapResolution, Rectangle};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    ratatui::run(|terminal| {
        terminal.draw(|frame| {
            let block = Block::bordered().title("Welcome");
            let greeting = Paragraph::new("Hello, Ratatui! 🐭")
                .centered()
                .yellow()
                .block(block);
            let canvas = Canvas::default()
                .block(Block::bordered().title("Canvas"))
                .x_bounds([-180.0, 180.0])
                .y_bounds([-90.0, 90.0])
                .paint(|ctx| {
                    ctx.draw(&Map {
                        resolution: MapResolution::High,
                        color: Color::White,
                    });
                    ctx.layer();
                    ctx.draw(&Line {
                        x1: 0.0,
                        y1: 10.0,
                        x2: 10.0,
                        y2: 10.0,
                        color: Color::White,
                    });
                    ctx.draw(&Rectangle {
                        x: 10.0,
                        y: 20.0,
                        width: 10.0,
                        height: 10.0,
                        color: Color::Red,
                    });
                });
            frame.render_widget(canvas, frame.area());
        })?;
        std::thread::sleep(std::time::Duration::from_secs(5));
        Ok(())
    })
}
