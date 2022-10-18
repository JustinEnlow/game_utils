pub struct Toggle{
    enabled: bool
}
impl Toggle{
    pub fn new(enabled: bool) -> Self{Self{enabled}}
    pub fn enabled(self: &Self) -> bool{self.enabled}
    pub fn toggle(self: &mut Self){self.enabled = !self.enabled}
}

// can this be replaced with a trait?
/*
    pub struct FlightControlSystem<'a, T>{
        linear_assist: T,
    }
    impl<'a, T> FlightControlSystem<'a, T>
        where T: Float + Toggle
    {
*/
pub trait TToggle{
    fn new(enabled: bool) -> Self;
    fn enabled(self: &Self) -> bool;
    fn toggle(self: &mut Self);//{self.enabled = !self.enabled()}
}



//pub struct Toggle{
//    enabled: bool
//}
//impl TToggle for Toggle{
//    fn new(enabled: bool) -> Self{Self{enabled}}
//    fn enabled(self: &Self) -> bool{self.enabled}
//    fn toggle(self: &mut Self){self.enabled = !self.enabled}
//}