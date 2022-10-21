//pub trait TToggle{
//    fn enabled(self: &Self) -> bool;
//    fn toggle(self: &mut Self);
//}

pub struct Toggle{
    enabled: bool
}
impl Toggle{
    pub fn new(enabled: bool) -> Self{Self{enabled}}
    pub fn enabled(self: &Self) -> bool{self.enabled}
    pub fn toggle(self: &mut Self){self.enabled = !self.enabled}
}