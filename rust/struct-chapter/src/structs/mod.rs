pub struct User {
  pub active: bool,
  pub username: String,
  pub email: String,
  pub sign_in_count: u64,
}

#[derive(Debug)]
pub struct Rectangle {
  pub width: u32,
  pub height: u32,
}

impl Rectangle {
  pub fn calculate_area(&self) -> u32 {
    self.width * self.height
  }

  pub fn width(&self) -> bool {
    self.width > 0
  }

  pub fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}