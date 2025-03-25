pub struct WindowInfo {
    pub owner: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl WindowInfo {
    /// Retourne vrai si le point (px, py) est dans le rectangle de la fenêtre.
    pub fn contains(&self, px: f64, py: f64) -> bool {
        px >= self.x && px <= (self.x + self.width) && py >= self.y && py <= (self.y + self.height)
    }
}
