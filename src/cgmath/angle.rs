use std::{
    f32::consts::{FRAC_1_PI, PI},
    ops::{Add, Neg, Sub},
};

const FRAC_1_180: f32 = 1.0 / 180.0;

/*
 */
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Radians(pub f32);

impl From<Degrees> for Radians {
    fn from(value: Degrees) -> Radians {
        Radians(
            PI * FRAC_1_180 * value.0
        )
    }
}

impl Add for Radians {
    type Output = Radians;
    fn add(
        self,
        other: Radians,
    ) -> Radians {
        Radians(self.0 + other.0)
    }
}

impl Sub for Radians {
    type Output = Radians;
    fn sub(
        self,
        other: Radians,
    ) -> Radians {
        Radians(self.0 - other.0)
    }
}

impl Neg for Radians {
    type Output = Radians;
    fn neg(self) -> Radians {
        Radians(-self.0)
    }
}

/*
*/
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Degrees(pub f32);

impl From<Radians> for Degrees {
    fn from(value: Radians) -> Degrees {
        Degrees(
            180.0 * FRAC_1_PI * value.0
        )
    }
}

impl Add for Degrees {
    type Output = Degrees;
    fn add(
        self,
        other: Degrees,
    ) -> Degrees {
        Degrees(self.0 + other.0)
    }
}

impl Sub for Degrees {
    type Output = Degrees;
    fn sub(
        self,
        other: Degrees,
    ) -> Degrees {
        Degrees(self.0 - other.0)
    }
}

impl Neg for Degrees {
    type Output = Degrees;
    fn neg(self) -> Degrees {
        Degrees(-self.0)
    }
}
