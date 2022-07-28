use std::fs;

use crate::colour::colour::Colour;

#[derive(Debug)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Vec<Colour>>,
}

trait NormaliseColour<T> {
    fn as_norm_colour(self) -> i32;
}

impl NormaliseColour<f32> for f32 {
    fn as_norm_colour(self) -> i32 {
        let normalised_self = (self * 255.0).ceil() as i32;
        if normalised_self >= 255 {
            255
        } else if normalised_self <= 0 {
            0
        } else {
            normalised_self
        }
    }
}

trait LineLengthLimited {
    fn limit_line_length(&self) -> String;
}

// TODO: think of a more elegant solution
impl LineLengthLimited for String {
    fn limit_line_length(&self) -> String {
        let mut count = 0;

        let word_list: Vec<String> = self
            .split(" ")
            .map(|word| {
                // check if word brings line over limit
                for _ in word.chars() {
                    count += 1;
                    if count % 70 == 0 {
                        return format!("\n{}", word);
                    }
                }
                // check if space brings line over limit
                count += 1;
                if count % 70 == 0 {
                    return format!("\n{}", word);
                }

                // return unchanged word otherwise
                format!("{}", word)
            })
            .collect();

        return word_list.join(" ");
    }
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![vec![Colour::default(); width]; height],
        }
    }

    pub fn save(&self, location: &str) -> () {
        fs::write(location, self.to_ppm()).expect("could not write ppm to file");
    }

    fn to_ppm(&self) -> String {
        let width_height = format!("{} {}", self.width, self.height);
        let pixel_grid = self.get_pixel_grid();
        let lines = vec!["P3", width_height.as_str(), "255", pixel_grid.as_str()];
        return lines
            .into_iter()
            .map(|line| format!("{}\n", line))
            .collect();
    }

    fn get_pixel_grid(&self) -> String {
        self.pixels
            .iter()
            .map(|pixel_col_line| {
                pixel_col_line
                    .iter()
                    .map(|colour| {
                        format!(
                            "{} {} {} ",
                            colour.red.as_norm_colour(),
                            colour.green.as_norm_colour(),
                            colour.blue.as_norm_colour()
                        )
                    })
                    .collect()
            })
            .map(|line: String| format!("{}\n", line.limit_line_length()))
            .collect()
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Option<Colour> {
        if x >= self.width || y >= self.height {
            None
        } else {
            return Some(self.pixels[y][x]);
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, colour: Colour) -> () {
        if x >= self.width || y >= self.height {
            println!(
                "Could not set pixel at ({},{}) in bounds of ({},{})",
                x, y, self.width, self.height
            );
        } else {
            self.pixels[y][x] = colour;
            ()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Canvas, LineLengthLimited};
    use crate::colour::colour::Colour;

    #[test]
    fn canvas_will_return_some_pixel_in_bounds() {
        let canvas = Canvas::new(5, 4);
        let in_bounds = canvas.get_pixel(4, 3);
        assert_eq!(in_bounds, Some(Colour::default()));
    }

    #[test]
    fn canvas_will_return_none_out_of_bounds() {
        let canvas = Canvas::new(5, 4);
        let sut = canvas.get_pixel(5, 3);
        assert_eq!(sut, None)
    }

    #[test]
    fn can_set_pixel_at_point() {
        let mut canvas = Canvas::new(5, 5);
        canvas.set_pixel(3, 3, Colour::new(1.0, 1.0, 1.0));
        assert_eq!(Colour::new(1.0, 1.0, 1.0), canvas.get_pixel(3, 3).unwrap());
    }

    #[test]
    fn canvas_to_ppm_returns_correct_headers() {
        let canvas = Canvas::new(5, 4);
        let ppm = canvas.to_ppm();
        let sut: Vec<&str> = ppm.split("\n").collect();
        assert_eq!(sut[0], "P3");
        assert_eq!(sut[1], "5 4");
        assert_eq!(sut[2], "255");
    }

    #[test]
    fn returns_correct_pixel_grid() {
        let mut canvas = Canvas::new(5, 3);
        canvas.set_pixel(0, 0, Colour::new(1.5, 0.0, 0.0));
        canvas.set_pixel(2, 1, Colour::new(0.0, 0.5, 0.0));
        canvas.set_pixel(4, 2, Colour::new(-0.5, 0.0, 1.0));
        let sut = canvas.get_pixel_grid();
        assert_eq!("255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 \n0 0 0 0 0 0 0 128 0 0 0 0 0 0 0 \n0 0 0 0 0 0 0 0 0 0 0 0 0 0 255 \n", sut)
    }

    #[test]
    fn line_will_be_limited_in_simple_case() {
        let input = String::from(
            "0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0",
        );
        let sut = input.limit_line_length();
        assert_eq!(
            String::from(
                "0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 \n0 0 0 0 0"
            ),
            sut
        )
    }

    #[test]
    fn line_will_be_limited_with_large_word_on_boundry() {
        let input = String::from(
            "0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 00000 0 0 0 0",
        );
        let sut = input.limit_line_length();
        assert_eq!(
            String::from(
                "0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 \n00000 0 0 0 0"
            ),
            sut
        )
    }

    #[test]
    fn line_will_be_limited_multuple_times() {
        let input = String::from(
            "0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 00000 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 00000 0 0 0 0",
        );
        let sut = input.limit_line_length();
        assert_eq!(
            String::from(
                "0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 \n00000 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 \n00000 0 0 0 0"
            ),
            sut
        )
    }

    #[test]
    fn lines_will_be_limited_in_real_example() {
        let mut canvas = Canvas::new(10, 2);
        for i in 0..10 {
            for j in 0..2 {
                canvas.set_pixel(i, j, Colour::new(1.0, 0.8, 0.6));
            }
        }
        let sut = canvas.get_pixel_grid();
        let expected = "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 \n153 255 204 153 255 204 153 255 204 153 255 204 153 \n255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 \n153 255 204 153 255 204 153 255 204 153 255 204 153 \n";
        assert_eq!(sut, expected);
    }
}
