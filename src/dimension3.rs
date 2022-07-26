//use std::ops::Neg;
//use crate::clamp;



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

//pub struct Vector3<T>{
//    x: T,
//    y: T,
//    z: T,
//    length: T,
//}
//impl<T> Vector3<T>
//    where T: Copy// + num::Float
//{}





// use Min to set a minimum value
// use Max to set a maximum value
// use Both to set a minimum and maximum value
//#[derive(Clone, Copy)]
//pub enum ClampType{
//    Min,
//    Max,
//    Both
//}



// version of Dimension3 that handles clamping dimension3 values in a variety of ways
// use low_clamp_value for ClampType::Min
// use high_clamp_value for ClampType::Max
// set whichever is unused to your types zero value
//#[derive(Clone, Copy)]
//pub struct ClampedDimension3<T>{
//    x: T,
//    y: T, 
//    z: T,
//    clamp_type: ClampType,
//    high_clamp_value: T,
//    low_clamp_value: T
//}
//impl<T> ClampedDimension3<T>
//    where T: std::cmp::PartialOrd + Copy + Neg<Output = T>
//{
//    pub fn new(x: T, y: T, z: T, clamp_type: ClampType, high_clamp_value: T, low_clamp_value: T) -> Self{
//        Self{
//            x, y, z,
//            clamp_type,
//            high_clamp_value,
//            low_clamp_value
//        }
//    }
//
//    pub fn x(self: &Self) -> T{self.x}
//    pub fn set_x(self: &mut Self, x: T){
//        match &self.clamp_type{
//            ClampType::Min => {
//                self.x = clamp::min(x, self.low_clamp_value);
//            },
//            ClampType::Max => {
//                self.x = clamp::max(x, self.high_clamp_value);
//            },
//            ClampType::Both => {
//                self.x = clamp::clamp_assym(x, self.high_clamp_value, self.low_clamp_value);
//            }
//        }
//    }
//    
//    pub fn y(self: &Self) -> T{self.y}
//    pub fn set_y(self: &mut Self, y: T){
//        match &self.clamp_type{
//            ClampType::Min => {
//                self.y = clamp::min(y, self.low_clamp_value);
//            },
//            ClampType::Max => {
//                self.y = clamp::max(y, self.high_clamp_value);
//            },
//            ClampType::Both => {
//                self.y = clamp::clamp_assym(y, self.high_clamp_value, self.low_clamp_value);
//            }
//        }
//    }
//    
//    pub fn z(self: &Self) -> T{self.z}
//    pub fn set_z(self: &mut Self, z: T){
//        match &self.clamp_type{
//            ClampType::Min => {
//                self.z = clamp::min(z, self.low_clamp_value);
//            },
//            ClampType::Max => {
//                self.z = clamp::max(z, self.high_clamp_value);
//            },
//            ClampType::Both => {
//                self.z = clamp::clamp_assym(z, self.high_clamp_value, self.low_clamp_value);
//            }
//        }
//    }
//}







// can prob use more tests, but this is all im doing for now...
//#[test]
//fn test_clamp_min(){
//    let expected_value: f32 = 0.0;
//    let mut test_var = ClampedDimension3::new(0.0, 0.0, 0.0, ClampType::Min, 0.0, 0.0);
//    test_var.set_x(-1.0);
//    assert!((test_var.x() - expected_value).abs() < 0.001);
//}

//#[test]
//fn test_clamp_max(){
//    let expected_value: f32 = 0.0;
//    let mut test_var = ClampedDimension3::new(0.0, 0.0, 0.0, ClampType::Max, 0.0, 0.0);
//    test_var.set_x(1.0);
//    assert!((test_var.x() - expected_value).abs() < 0.001);
//}

//#[test]
//fn test_clamp_both(){
//    let high_expected_value: f32 = 1.0;
//    let low_expected_value: f32 = -1.0;
//    let mut test_var = ClampedDimension3::new(0.0, 0.0, 0.0, ClampType::Both, 1.0, -1.0);
//    test_var.set_x(2.0);
//    test_var.set_y(-2.0);
//    assert!((test_var.x() - high_expected_value).abs() < 0.001);
//    assert!((test_var.y() - low_expected_value).abs() < 0.001);
//}