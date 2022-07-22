/// a struct representing a linear and a rotational axis
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
    pub fn default(default: T) -> Self{
        Self {
            linear: default, 
            rotational: default
        }
    }

    //pub fn linear(self: &Self) -> T{self.linear}
    pub fn linear/*_ref*/(self: &Self) -> &T{&self.linear}
    pub fn linear_mut(self: &mut Self) -> &mut T{&mut self.linear}
    
    //pub fn rotational(self: &Self) -> T{self.rotational}
    pub fn rotational/*_ref*/(self: &Self) -> &T{&self.rotational}
    pub fn rotational_mut(self: &mut Self) -> &mut T{&mut self.rotational}
}



#[derive(Clone, Copy)]
pub struct AxisContribution<T>{
    positive: T,
    negative: T
}
impl<T> AxisContribution<T>
    where T: Copy
{
    pub fn new(positive: T, negative: T) -> Self{
        Self{positive, negative}
    }

    pub fn positive(self: &Self) -> /*&*/T{/*&*/self.positive}
    pub fn positive_mut(self: &mut Self) -> &mut T{&mut self.positive}
    pub fn set_positive(self: &mut Self, value: T){self.positive = value}

    pub fn negative(self: &Self) -> /*&*/T{/*&*/self.negative}
    pub fn negative_mut(self: &mut Self) -> &mut T{&mut self.negative}
    pub fn set_negative(self: &mut Self, value: T){self.negative = value}
}