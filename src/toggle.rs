// is trait or struct better for this use case?

pub trait Toggle{
    //fn new(enabled: bool) -> Self;
    fn enabled(self: &Self) -> bool;
    fn toggle(self: &mut Self);//{self.enabled = !self.enabled()}
}

//pub struct Toggle{
//    enabled: bool
//}
//impl Toggle{
//    pub fn new(enabled: bool) -> Self{Self{enabled}}
//    pub fn enabled(self: &Self) -> bool{self.enabled}
//    pub fn toggle(self: &mut Self){self.enabled = !self.enabled}
//}





/*
examples:

pub struct Power{
    enabled: bool
}
impl Toggle for Power{
    pub fn new(enabled: bool) -> Self{Self{enabled}}
    pub fn enabled(self: &Self) -> bool{self.enabled}
    pub fn toggle(self: &mut Self){self.enabled = !self.enabled}
}

pub struct GSafety{
    enabled: bool,
    max_acceleration: ControlAxis<Dimension3<AxisContribution<f32>>>
}
impl Toggle for GSafety{
    pub fn new(enabled: bool) -> Self{Self{enabled}}
    pub fn enabled(self: &Self) -> bool{self.enabled}
    pub fn toggle(self: &mut Self){self.enabled = !self.enabled}
}
*/