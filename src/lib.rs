use control_axis::ControlAxis;
use dimension3::Dimension3;
use std::ops::Add;

pub mod control_axis;
pub mod toggle;
pub mod dimension3;
pub mod clamp;

pub fn sum_d3_control_axes<T>(
    input1: ControlAxis<Dimension3<T>>, 
    input2: ControlAxis<Dimension3<T>>
) -> ControlAxis<Dimension3<T>>
    where T: Add<Output = T>
        + Copy
{
    ControlAxis::new(
        Dimension3::new(
            input1.linear().x() + input2.linear().x(),
            input1.linear().y() + input2.linear().y(),
            input1.linear().z() + input2.linear().z(),
        ),
        Dimension3::new(
            input1.rotational().x() + input2.rotational().x(),
            input1.rotational().y() + input2.rotational().y(),
            input1.rotational().z() + input2.rotational().z(),
        )
    )
}