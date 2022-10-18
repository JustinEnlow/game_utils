use std::ops::{Mul, Add};

//kinematics
    //d = displacement
    //v = velocity
    //a = acceleration
    //t = time

    //vsubt = vsubi + at
    //velocity_at_some_time = initial_velocity + (acceleration * time)
    pub fn velocity_at_time<T>(initial_velocity: T, acceleration: T, time: T) -> T
        where T: Mul<Output = T> + Add<Output = T>
    {
        initial_velocity + (acceleration * time)
    }
    #[test]
    fn test_velocity_at_time(){
        let acceleration = 2.5; //meters / (second^2)
        let time = 10.0; //seconds
        let initial_velocity = 0.0; //meters / second
        assert!(velocity_at_time(initial_velocity, acceleration, time) == 25.0);
    }

    //xsubt = xsub0 + (vsubi * t) + (1/2 * a) * (t^2)
    //position_relative_to_origin = initial_position + (initial_velocity * time) + (1/2 * acceleration) * (time^2)
    //distance traveled?
    pub fn position_from_origin<T>(initial_position: T, initial_velocity: T, time: T, acceleration: T, one_half: T) -> T
        where T: Add<Output = T> + Mul<Output = T> + Copy
    {
        initial_position + (initial_velocity * time) + ((one_half * acceleration) * (time * time))
    }
    #[test]
    fn test_position_from_origin(){
        let acceleration = 2.5; //meters / (second^2)
        let time = 10.0; //seconds
        let initial_velocity = 0.0; //meters / second
        let initial_position = 0.0; //meters
        let one_half = 0.5;
        assert!(position_from_origin(initial_position, initial_velocity, time, acceleration, one_half) == 125.0/*meters traveled*/);
    }

    //vsubf^2 = vsubi^2 + (2 * a) * d
    //velocity^2 = initial_velocity^2 + (2 * acceleration) * displacement

    //d = ((vsubf + vsubi) / 2) * t
    //position = average_velocity * time

    //v = (vsubf + vsubi) / 2
    //average velocity = (final_velocity + initial_velocity) / 2