use super::List;
use super::New;

#[derive(New, Debug, PartialEq, Eq)]
pub struct RGB(u8, u8, u8);

/// TODO: Make this smarter lmao
#[derive(New)]
pub struct Screenshot {
    width: u32,
    height: u32,
    screen: List<RGB>,
}

impl std::fmt::Debug for Screenshot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Screenshot ({}x{})", self.width, self.height))
    }
}
