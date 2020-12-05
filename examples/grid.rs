use crossterm::event::{Event, KeyCode};
use semtext::{layout, Edge, Layout, Screen, Widget};
use semtext::widget::{Border, Label, LineStyle, Spacer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut screen = Screen::new()?;
    screen.set_title("Layout Test")?;
    let s0 = Spacer::default();
    let s1 = Spacer::default().with_fill('.')?;
    let b0 = Border::default()
        .with_edges(Edge::ALL)
        .with_accents(Edge::BOTTOM_RIGHT)
        .with_line_style(LineStyle::Double);
    let a = Label::new("This is a bit of test text inside of a label");
    let b = Label::new("This label is on the right side");
    loop {
        let layout = layout!(screen.bbox(),
            [s0 s0 s0 b0],
            [ a s1  b b0],
        )?;
        screen.render(&layout)?;
        match screen.event()? {
            Event::Key(key) => if key.code == KeyCode::Esc { break; },
            _ => {},
        }
    }
    Ok(())
}
