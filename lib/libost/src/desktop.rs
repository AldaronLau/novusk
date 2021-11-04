use alloc::vec::Vec;
use crate::graphics::graphics_pixel;

#[derive(Copy, Clone)]
pub struct DesktopIcon {
    pub size: (usize, usize),
    pub color: usize,
}

impl DesktopIcon {
    pub fn new(icon_size: (usize, usize), color: usize) -> Self {
        return DesktopIcon {
            size: icon_size,
            color: color,
        };
    }

    pub fn display(&mut self) {

    }
}

pub struct Desktop {
    pub size: (usize, usize),
    pub color: usize,
    // TODO: Make a mouse cursor struct in the future for mouse info
    pub cursor: bool,
    pub icons: Option<Vec<DesktopIcon>>,
}

impl Desktop {
    pub fn new(desktop_size: (usize, usize), background_color: usize, desktop_icons: Option<Vec<DesktopIcon>>) -> Self {
        return Desktop {
            size: desktop_size,
            color: background_color,
            cursor: false,
            icons: desktop_icons,
        };
    }

    pub fn init(&mut self) {
        let (x, y) = self.size;

        for dy in 0..y {
            for dx in 0..x {
                graphics_pixel(dx, dy, self.color);
            }
        }
    }
}