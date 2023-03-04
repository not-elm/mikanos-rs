#[derive(Debug)]
pub struct Color {
    _r: u8,
    _g: u8,
    _b: u8,
}


impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { _r: r, _g: g, _b: b }
    }

    pub fn r(&self) -> u8 {
        self._r
    }


    pub fn g(&self) -> u8 {
        self._g
    }


    pub fn b(&self) -> u8 {
        self._b
    }
}