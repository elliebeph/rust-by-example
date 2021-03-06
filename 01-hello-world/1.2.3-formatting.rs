use std::fmt::{self, Formatter, Display};

struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl Display for City {
    // `f` is a buffer that the formatted string is written to.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // `write!` is like `format!` but writes the formatted string to a
        // buffer.
        write!(f, "{}: {:.3}°{} {:.3}°{}",
               self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Colour {
    red:   u8,
    green: u8,
    blue:  u8,
}

impl Display for Colour {
    fn fmt(&self, f: &mut Formatter) ->fmt::Result {
        write!(f,
               "RGB ({red}, {green}, {blue}) {red:#04X}{green:02X}{blue:02X}",
               red   = self.red,
               green = self.green,
               blue  = self.blue)
    }
}

fn main() {
    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", *city);
    }

    for colour in [
        Colour { red: 128, green: 255, blue: 90 },
        Colour { red: 0, green: 3, blue: 254 },
        Colour { red: 0, green: 0, blue: 0 },
    ].iter() {
        println!("{}", *colour);
    }
}
