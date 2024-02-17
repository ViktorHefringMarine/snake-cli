pub trait Render {
    fn render(cell: (u16, u16)) -> Result<(), crossterm::ErrorKind>;
}
