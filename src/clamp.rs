use std::ops::{Neg, Mul};



/// clamp input value between two values, where the clamp values are not a positive and negative of the same number
/// low_clamp_value <= input_value <= high_clamp_value
pub fn clamp_assym<T: PartialOrd + Neg<Output = T> + Copy>(input_value: T, high_clamp_value: T, low_clamp_value: T) -> T{
    if input_value > high_clamp_value{
        high_clamp_value
    }
    else if input_value < low_clamp_value{
        low_clamp_value
    }
    else{
        input_value
    }
}

/// clamp input value between two value, where the clamp values are a positive and negative of the same number
/// -clamp_value <= input_value <= clamp_value
pub fn clamp<T: PartialOrd + Neg<Output = T> + Copy>(input_value: T, clamp_value: T) -> T{
    if input_value > clamp_value{
        clamp_value
    }
    else if input_value < -clamp_value{
        -clamp_value
    }
    else{
        input_value
    }
}

/// clamp input value from going below a minimum value
pub fn min<T: PartialOrd + Copy>(input_value: T, min_value: T) -> T{
    if input_value < min_value{
        min_value
    }
    else{
        input_value
    }
}

/// clamp input value from going above a maximum value
pub fn max<T: PartialOrd + Copy>(input_value: T, max_value: T) -> T{
    if input_value > max_value{
        max_value
    }
    else{
        input_value
    }
}

pub fn cmp_mul_assym<T: PartialOrd + Mul<Output = T> + Copy>(
    input_value: T, compare_value: T, high_mul_value: T, low_mul_value: T
) -> T{
    if input_value > compare_value{
        input_value * high_mul_value
    }
    else if input_value < compare_value{
        input_value * low_mul_value
    }
    else{compare_value}
}



///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////



#[test]
fn values_clamp_on_high_range(){
    let out_of_range: f64 = 2.0;
    let expected_output = 1.0;
    let sym_clamped_value = clamp(out_of_range, 1.0);
    let assym_clamped_value = clamp_assym(out_of_range, 1.0, -1.0);
    let max_clamped_value = max(out_of_range, 1.0);
    assert!((sym_clamped_value - expected_output).abs() < 0.001);
    assert!((assym_clamped_value - expected_output).abs() < 0.001);
    assert!((max_clamped_value - expected_output).abs() < 0.001);
}

#[test]
fn values_clamp_on_low_range(){
    let out_of_range: f64 = -2.0;
    let expected_output = -1.0;
    let sym_clamped_value = clamp(out_of_range, 1.0);
    let assym_clamped_value = clamp_assym(out_of_range, 1.0, -1.0);
    let min_clamped_value = min(out_of_range, -1.0);
    assert!((sym_clamped_value - expected_output).abs() < 0.001);
    assert!((assym_clamped_value - expected_output).abs() < 0.001);
    assert!((min_clamped_value - expected_output).abs() < 0.001);
}

#[test]
fn values_dont_clamp_when_within_range(){
    let in_range: f64 = 0.5;
    let expected_output = 0.5;
    let sym_clamped_value = clamp(in_range, 1.0);
    let assym_clamped_value = clamp_assym(in_range, 1.0, -1.0);
    let min_clamped_value = min(in_range, 0.0);
    let max_clamped_value = max(in_range, 1.0);
    assert!((sym_clamped_value - expected_output).abs() < 0.001);
    assert!((assym_clamped_value - expected_output).abs() < 0.001);
    assert!((min_clamped_value - expected_output).abs() < 0.001);
    assert!((max_clamped_value - expected_output).abs() < 0.001);
}