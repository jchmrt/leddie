// controller.rs --- leddie

// Copyright (C) 2021 Jochem Raat <jochem@invulns.nl>

// Author: Jochem Raat <jochem@invulns.nl>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use rs_ws281x::ChannelBuilder;
use rs_ws281x::Controller;
use rs_ws281x::ControllerBuilder;
use rs_ws281x::StripType;

pub struct LeddieScreen {
    width: u8,
    height: u8,
    controller: Controller,
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Clone, Copy)]
pub struct Coordinate {
    pub x: u8,
    pub y: u8,
}

#[derive(Debug)]
pub enum CoordError {
    XCoordError { x: u8, width: u8 },
    YCoordError { y: u8, height: u8 },
}

use CoordError::XCoordError;
use CoordError::YCoordError;

impl Coordinate {
    pub fn new(x: u8, y: u8) -> Coordinate {
        Coordinate { x, y }
    }
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }
}

/// Implements control over the screen of LEDs
impl LeddieScreen {
    pub fn new(width: u8, height: u8) -> LeddieScreen {
        let controller = ControllerBuilder::new()
            .freq(800_000)
            .dma(10)
            .channel(
                0,
                ChannelBuilder::new()
                    .pin(18)
                    .count((width * height) as i32)
                    .strip_type(StripType::Ws2811Gbr)
                    .brightness(155)
                    .build(),
            )
            .build()
            .expect("Problem creating LED controller");

        LeddieScreen {
            width,
            height,
            controller,
        }
    }

    pub fn render(&mut self) {
        self.controller.render().expect("Error rendering LEDs");
    }

    pub fn set_pixel(&mut self, coord: Coordinate, color: Color) -> Result<(), CoordError> {
        if coord.x >= self.width {
            Err(XCoordError {
                x: coord.x,
                width: self.width,
            })
        } else if coord.y >= self.height {
            Err(YCoordError {
                y: coord.y,
                height: self.height,
            })
        } else {
            let leds = self.controller.leds_mut(0);

            let i = (self.width * self.height) - coord.y * self.width - 1;

            // If 1, show to outside. If 0, show to inside.
            let mirror_type = 1;

            let i = if coord.y % 2 == mirror_type {
                i - coord.x
            } else {
                i - (self.width - 1 - coord.x)
            };

            leds[usize::from(i)] = [color.r, color.g, color.b, 0];

            Ok(())
        }
    }

    pub fn set_brightness(&mut self, brightness: u8) {
        self.controller.set_brightness(0, brightness);
    }
}
