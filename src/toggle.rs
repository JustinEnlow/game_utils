pub struct Toggle{
    enabled: bool
}
impl Toggle{
    pub fn new() -> Self{Self{enabled: true}}
    pub fn enabled(self: &Self) -> bool{self.enabled}
    pub fn toggle(self: &mut Self){self.enabled = !self.enabled}
}