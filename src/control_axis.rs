#[derive(Clone, Copy)]
pub struct ControlAxis<T>{
    linear: T,
    rotational: T,
}
impl<T> ControlAxis<T>
    where T: Copy
{
    pub fn new(linear: T, rotational: T) -> Self{
        Self{linear, rotational}
    }

    //pub fn linear(self: &Self) -> T{self.linear}
    pub fn linear/*_ref*/(self: &Self) -> &T{&self.linear}
    pub fn linear_mut(self: &mut Self) -> &mut T{&mut self.linear}
    
    //pub fn rotational(self: &Self) -> T{self.rotational}
    pub fn rotational/*_ref*/(self: &Self) -> &T{&self.rotational}
    pub fn rotational_mut(self: &mut Self) -> &mut T{&mut self.rotational}
}