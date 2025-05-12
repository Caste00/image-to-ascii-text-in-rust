use image::Rgba;

pub struct Pixel {
    width: u32,
    height: u32,
    color: Rgba<u8>,
}

impl Pixel {
    pub fn new(width: u32, height: u32, color: Rgba<u8>) -> Self {
        Self {
            width,
            height,
            color
        }
    }
    
    pub fn get_color(&self) -> Rgba<u8> {
        self.color
    }
    
    pub fn get_position(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}