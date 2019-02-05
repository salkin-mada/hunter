use ::rayon::prelude::*;

use std::io::BufRead;

use crate::coordinates::{Coordinates, Position, Size};
use crate::files::File;
use crate::term::sized_string;
use crate::widget::Widget;

#[derive(PartialEq)]
pub struct TextView {
    pub lines: Vec<String>,
    pub buffer: String,
    pub coordinates: Coordinates,
}

impl TextView {
    pub fn new_from_file(file: &File) -> TextView {
        let file = std::fs::File::open(&file.path).unwrap();
        let file = std::io::BufReader::new(file);
        let lines = file.lines().map(|line|
                                     line.unwrap()
                                     .replace("\t", "    ")).collect();

        TextView {
            lines: lines,
            buffer: String::new(),
            coordinates: Coordinates::new(),
        }
    }
}

impl Widget for TextView {
    fn get_size(&self) -> &Size {
        &self.coordinates.size
    }
    fn set_size(&mut self, size: Size) {
        self.coordinates.size = size;
    }
    fn get_position(&self) -> &Position {
        &self.coordinates.position
    }
    fn set_position(&mut self, pos: Position) {
        self.coordinates.position = pos;
    }
    fn get_coordinates(&self) -> &Coordinates {
        &self.coordinates
    }
    fn set_coordinates(&mut self, coordinates: &Coordinates) {
        self.coordinates = coordinates.clone();
        self.refresh();
    }
    fn render_header(&self) -> String {
        "".to_string()
    }
    fn refresh(&mut self) {
        let (xsize, ysize) = self.get_size().size();
        let (xpos, ypos) = self.get_coordinates().position().position();

        self.buffer = self.get_clearlist() +
            &self
            .lines
            .par_iter()
            .take(ysize as usize)
            .enumerate()
            .map(|(i, line)| {
                format!(
                    "{}{}{}",
                    crate::term::goto_xy(xpos, i as u16 + ypos),
                    crate::term::reset(),
                    sized_string(&line, xsize))
            })
            .collect::<String>();
    }

    fn get_drawlist(&self) -> String {
        self.buffer.clone()
    }
}
