#[derive(Clone)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Colour {
    pub fn new() -> Colour {
        Colour {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }
    }

    pub fn new_opaqe(r: u8, g: u8, b: u8) -> Colour {
        Colour {
            r: r,
            g: g,
            b: b,
            a: 255,
        }
    }

    pub fn new_alpha(r: u8, g: u8, b: u8, a: u8) -> Colour {
        Colour {
            r: r,
            g: g,
            b: b,
            a: a,
        }
    }
}
