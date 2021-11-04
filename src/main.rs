// main.rs --- leddie

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

mod controller;
mod reader;

use controller::LeddieScreen;
use reader::LeddieReader;

const SCR_WIDTH: u8 = 15;
const SCR_HEIGHT: u8 = 10;

fn main() {
    let scr = LeddieScreen::new(SCR_WIDTH, SCR_HEIGHT);
    let mut reader = LeddieReader::new(scr);

    reader.run();
}
