use std::ops::Neg;
use crate::clamp;



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


#[derive(Clone, Copy)]
pub struct ClampedDimension3<T>{
    x: T,
    y: T,
    z: T,
    high_clamp_value: T,
    low_clamp_value: T,
}
impl<T> ClampedDimension3<T>
    where
        T: std::cmp::PartialOrd
        + Neg<Output = T>
        + Copy
{
    pub fn new(x: T, y: T, z: T, high_clamp_value: T, low_clamp_value: T) -> Self{
        Self{
            x: clamp::clamp_assym(x, high_clamp_value, low_clamp_value), 
            y: clamp::clamp_assym(y, high_clamp_value, low_clamp_value),
            z: clamp::clamp_assym(z, high_clamp_value, low_clamp_value),
            high_clamp_value,
            low_clamp_value,
        }
    }

    pub fn x(self: &Self) -> T{self.x}
    pub fn set_x(self: &mut Self, x: T){
        self.x = clamp::clamp_assym(x, self.high_clamp_value, self.low_clamp_value);
    }
    
    pub fn y(self: &Self) -> T{self.y}
    pub fn set_y(self: &mut Self, y: T){
        self.y = clamp::clamp_assym(y, self.high_clamp_value, self.low_clamp_value);
    }
    
    pub fn z(self: &Self) -> T{self.z}
    pub fn set_z(self: &mut Self, z: T){
        self.z = clamp::clamp_assym(z, self.high_clamp_value, self.low_clamp_value);
    }
}

#[derive(Clone, Copy)]
pub struct MinDimension3<T>{
    x: T,
    y: T,
    z: T,
    min_clamp_value: T
}
impl <T> MinDimension3<T>
    where
        T: std::cmp::PartialOrd
        + Copy
{
    pub fn new(x: T, y: T, z: T, min_clamp_value: T) -> Self{
        Self{
            x: clamp::min(x, min_clamp_value),
            y: clamp::min(y, min_clamp_value),
            z: clamp::min(z, min_clamp_value),
            min_clamp_value
        }
    }

    pub fn x(self: &Self) -> T{self.x}
    pub fn set_x(self: &mut Self, x: T){
        self.x = clamp::min(x, self.min_clamp_value);
    }

    pub fn y(self: &Self) -> T{self.y}
    pub fn set_y(self: &mut Self, y: T){
        self.y = clamp::min(y, self.min_clamp_value);
    }

    pub fn z(self: &Self) -> T{self.z}
    pub fn set_z(self: &mut Self, z: T){
        self.z = clamp::min(z, self.min_clamp_value);
    }
}

#[derive(Clone, Copy)]
pub struct MaxDimension3<T>{
    x: T,
    y: T,
    z: T,
    max_clamp_value: T
}
impl <T> MaxDimension3<T>
    where
        T: std::cmp::PartialOrd
        + Copy
{
    pub fn new(x: T, y: T, z: T, max_clamp_value: T) -> Self{
        Self{
            x: clamp::max(x, max_clamp_value),
            y: clamp::max(y, max_clamp_value),
            z: clamp::max(z, max_clamp_value),
            max_clamp_value
        }
    }

    pub fn x(self: &Self) -> T{self.x}
    pub fn set_x(self: &mut Self, x: T){
        self.x = clamp::max(x, self.max_clamp_value);
    }

    pub fn y(self: &Self) -> T{self.y}
    pub fn set_y(self: &mut Self, y: T){
        self.y = clamp::max(y, self.max_clamp_value);
    }

    pub fn z(self: &Self) -> T{self.z}
    pub fn set_z(self: &mut Self, z: T){
        self.z = clamp::max(z, self.max_clamp_value);
    }
}