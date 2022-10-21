/// struct for 3 dimensions of any type
#[derive(Clone, Copy)]
pub struct Dimension3<T>{
    x: T,
    y: T,
    z: T,
}
impl<T> Dimension3<T>
    where T: Copy
{
    pub fn new(x: T, y: T, z: T) -> Self{
        Self{x, y, z}
    }
    pub fn default(zero: T) -> Self{
        Self{x: zero, y: zero, z: zero}
    }
    
    pub fn x(self: &Self) -> T{self.x}
    pub fn x_mut(self: &mut Self) -> &mut T{&mut self.x}    //needed for setting PID struct fields
    pub fn set_x(self: &mut Self, x: T){self.x = x}
    
    pub fn y(self: &Self) -> T{self.y}
    pub fn y_mut(self: &mut Self) -> &mut T{&mut self.y}    //
    pub fn set_y(self: &mut Self, y: T){self.y = y}
    
    pub fn z(self: &Self) -> T{self.z}
    pub fn z_mut(self: &mut Self) -> &mut T{&mut self.z}    //
    pub fn set_z(self: &mut Self, z: T){self.z = z}
}