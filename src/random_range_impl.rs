//stuff goes here

use crate::random_traits::Random;
use crate::random_traits::RandomRange;

impl RandomRange for f32 {
    fn random_range(min: f32, max: f32) -> f32 {
        let diff = max - min;
        min + (diff * f32::random())
    }
}

impl RandomRange for f64 {
    fn random_range(min: f64, max: f64) -> f64 {
        let diff = max - min;
        min + (diff * f64::random())
    }
}

impl RandomRange for i8 {
    fn random_range(min: i8, max: i8) -> i8 {
        fastrand::i8(min..=max)
    }
}

impl RandomRange for i16 {
    fn random_range(min: i16, max: i16) -> i16 {
        fastrand::i16(min..=max)
    }
}

impl RandomRange for i32 {
    fn random_range(min: i32, max: i32) -> i32 {
        fastrand::i32(min..=max)
    }
}

impl RandomRange for i64 {
    fn random_range(min: i64, max: i64) -> i64 {
        fastrand::i64(min..=max)
    }
}

impl RandomRange for i128 {
    fn random_range(min: i128, max: i128) -> i128 {
        fastrand::i128(min..=max)
    }
}

impl RandomRange for u8 {
    fn random_range(min: u8, max: u8) -> u8 {
        fastrand::u8(min..=max)
    }
}

impl RandomRange for u16 {
    fn random_range(min: u16, max: u16) -> u16 {
        fastrand::u16(min..=max)
    }
}

impl RandomRange for u32 {
    fn random_range(min: u32, max: u32) -> u32 {
        fastrand::u32(min..=max)
    }
}

impl RandomRange for u64 {
    fn random_range(min: u64, max: u64) -> u64 {
        fastrand::u64(min..=max)
    }
}

impl RandomRange for u128 {
    fn random_range(min: u128, max: u128) -> u128 {
        fastrand::u128(min..=max)
    }
}

impl RandomRange for usize {
    fn random_range(min: usize, max: usize) -> usize {
        fastrand::usize(min..=max)
    }
}

#[cfg(feature = "spatial2d")]
mod spatial2d {
    use crate::random_range_impl::RandomRange;
    use crate::random_traits::Random;
    use rantz_spatial2d::prelude::*;

    impl RandomRange for Degrees {
        fn random_range(min: Degrees, max: Degrees) -> Degrees {
            let diff = max - min;
            min + (diff * f32::random())
        }
    }

    impl RandomRange for Radians {
        fn random_range(min: Radians, max: Radians) -> Radians {
            let diff = max - min;
            min + (diff * f32::random())
        }
    }
}

#[cfg(feature = "bevy")]
mod bevy {
    use crate::random_range_impl::RandomRange;
    use crate::random_traits::Random;
    use bevy::prelude::*;

    impl RandomRange for Vec2 {
        fn random_range(min: Vec2, max: Vec2) -> Vec2 {
            let diff = max - min;
            min + (diff * Vec2::new(f32::random(), f32::random()))
        }
    }

    impl RandomRange for Vec3 {
        fn random_range(min: Vec3, max: Vec3) -> Vec3 {
            let diff = max - min;
            min + (diff * Vec3::new(f32::random(), f32::random(), f32::random()))
        }
    }

    impl RandomRange for Vec4 {
        fn random_range(min: Vec4, max: Vec4) -> Vec4 {
            let diff = max - min;
            min + (diff * Vec4::new(f32::random(), f32::random(), f32::random(), f32::random()))
        }
    }

    impl RandomRange for UVec2 {
        fn random_range(min: UVec2, max: UVec2) -> UVec2 {
            let x_diff = max.x - min.x;
            let y_diff = max.y - min.y;
            let new_x = (x_diff as f32 * f32::random()) as u32;
            let new_y = (y_diff as f32 * f32::random()) as u32;
            min + UVec2::new(new_x, new_y)
        }
    }

    impl RandomRange for UVec3 {
        fn random_range(min: UVec3, max: UVec3) -> UVec3 {
            let x_diff = max.x - min.x;
            let y_diff = max.y - min.y;
            let z_diff = max.z - min.z;
            let new_x = (x_diff as f32 * f32::random()) as u32;
            let new_y = (y_diff as f32 * f32::random()) as u32;
            let new_z = (z_diff as f32 * f32::random()) as u32;
            min + UVec3::new(new_x, new_y, new_z)
        }
    }

    impl RandomRange for UVec4 {
        fn random_range(min: UVec4, max: UVec4) -> UVec4 {
            let x_diff = max.x - min.x;
            let y_diff = max.y - min.y;
            let z_diff = max.z - min.z;
            let w_diff = max.w - min.w;
            let new_x = (x_diff as f32 * f32::random()) as u32;
            let new_y = (y_diff as f32 * f32::random()) as u32;
            let new_z = (z_diff as f32 * f32::random()) as u32;
            let new_w = (w_diff as f32 * f32::random()) as u32;
            min + UVec4::new(new_x, new_y, new_z, new_w)
        }
    }

    impl RandomRange for IVec2 {
        fn random_range(min: IVec2, max: IVec2) -> IVec2 {
            let x_diff = max.x - min.x;
            let y_diff = max.y - min.y;
            let new_x = (x_diff as f32 * f32::random()) as i32;
            let new_y = (y_diff as f32 * f32::random()) as i32;
            min + IVec2::new(new_x, new_y)
        }
    }

    impl RandomRange for IVec3 {
        fn random_range(min: IVec3, max: IVec3) -> IVec3 {
            let x_diff = max.x - min.x;
            let y_diff = max.y - min.y;
            let z_diff = max.z - min.z;
            let new_x = (x_diff as f32 * f32::random()) as i32;
            let new_y = (y_diff as f32 * f32::random()) as i32;
            let new_z = (z_diff as f32 * f32::random()) as i32;
            min + IVec3::new(new_x, new_y, new_z)
        }
    }

    impl RandomRange for IVec4 {
        fn random_range(min: IVec4, max: IVec4) -> IVec4 {
            let x_diff = max.x - min.x;
            let y_diff = max.y - min.y;
            let z_diff = max.z - min.z;
            let w_diff = max.w - min.w;
            let new_x = (x_diff as f32 * f32::random()) as i32;
            let new_y = (y_diff as f32 * f32::random()) as i32;
            let new_z = (z_diff as f32 * f32::random()) as i32;
            let new_w = (w_diff as f32 * f32::random()) as i32;
            min + IVec4::new(new_x, new_y, new_z, new_w)
        }
    }
}
