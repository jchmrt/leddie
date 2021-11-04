// reader.rs --- leddie

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

use std::error::Error;
use std::ffi::CString;
use std::fmt;
use std::fmt::Display;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use super::controller::Color;
use super::controller::Coordinate;
use super::controller::LeddieScreen;

pub struct LeddieReader {
    file: File,
    scr: LeddieScreen,
}

#[derive(FromPrimitive, Debug, PartialEq)]
enum CommandType {
    SetPixel = 0,
    SetScreen = 1,
    Render = 2,
}

#[derive(Debug)]
pub enum Command {
    SetPixel { x: u8, y: u8, r: u8, g: u8, b: u8 },

    // TODO:
    // SetScreen {
    // },

    Render,
}

#[derive(Debug)]
pub struct ParseError;

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error parsing command")
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        "Error parsing command"
    }
}

const PIPE_FILE_NAME: &str = "/tmp/leddie";

impl LeddieReader {
    fn init_named_pipe() {
        if !Path::new(PIPE_FILE_NAME).exists() {
            let path: CString = CString::new(PIPE_FILE_NAME).unwrap();

            unsafe {
                libc::umask(0);
                let res = libc::mkfifo(path.as_ptr(), 0o666);

                if res < 0 {
                    panic!("Failed to create named pipe");
                }
            }
        }
    }

    pub fn new(scr: LeddieScreen) -> Self {
        LeddieReader::init_named_pipe();

        LeddieReader {
            file: OpenOptions::new()
                .read(true)
                .write(true)
                .open(PIPE_FILE_NAME)
                .expect("Failed to open named pipe"),
            scr,
        }
    }

    fn parse_command_type(&mut self) -> Result<CommandType, Box<dyn Error>> {
        let mut byte: [u8; 1] = [0];
        self.file.read_exact(&mut byte)?;
        match FromPrimitive::from_u8(byte[0]) {
            Some(r) => Ok(r),
            None => Err(Box::new(ParseError)),
        }
    }

    fn parse_set_pixel(&mut self) -> Result<Command, Box<dyn Error>> {
        let mut bytes: [u8; 5] = [0; 5];
        self.file.read_exact(&mut bytes)?;
        Ok(Command::SetPixel {
            x: bytes[0],
            y: bytes[1],
            r: bytes[2],
            g: bytes[3],
            b: bytes[4],
        })
    }

    fn parse(&mut self) -> Result<Command, Box<dyn Error>> {
        let cmd_type = self.parse_command_type()?;
        match cmd_type {
            CommandType::SetPixel => self.parse_set_pixel(),
            CommandType::SetScreen => Err(Box::new(ParseError)), // TODO: implement
            CommandType::Render => Ok(Command::Render),
        }
    }

    pub fn run(&mut self) {
        loop {
            match self.parse() {
                Ok(Command::SetPixel { x, y, r, g, b }) => {
                    if self
                        .scr
                        .set_pixel(Coordinate::new(x, y), Color::new(r, g, b))
                        .is_err()
                    {
                        println!("Failed to set invalid pixel at {},{}", x, y);
                    }
                }
                Ok(Command::Render) => {
                    self.scr.render();
                }
                _ => {
                    println!("parse error in command");
                }
            };
        }
    }
}
