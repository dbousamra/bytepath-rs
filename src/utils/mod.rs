use ggez::graphics;

pub fn BACKROUND_COLOR() -> graphics::Color {
  graphics::Color::from_rgb(16, 16, 16)
}
pub fn AMMO_COLOR() -> graphics::Color {
  graphics::Color::from_rgb(123, 200, 164)
}
pub fn BOOST_COLOR() -> graphics::Color {
  graphics::Color::from_rgb(76, 195, 217)
}
pub fn HP_COLOR() -> graphics::Color {
  graphics::Color::from_rgb(241, 103, 69)
}

pub fn rect_to_polygon(rect: graphics::Rect) -> Vec<graphics::Point2> {
  let x1 = rect.x;
  let x2 = rect.x + rect.w;
  let y1 = rect.y;
  let y2 = rect.y + rect.h;
  vec![
    graphics::Point2::new(x1, y1),
    graphics::Point2::new(x2, y1),
    graphics::Point2::new(x2, y2),
    graphics::Point2::new(x1, y2),
  ]
}
