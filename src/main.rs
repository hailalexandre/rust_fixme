use std::fmt::{self, Formatter, Display};

/* Demonstrates printing of a user defined struct using println! macro.*/

/// Sets up the frame for the name of the city, and well as the latitude and longitude coordinates, specifying the data types
struct City {
    name: &'static str,
    // Latitude
    lat: f32,
    // Longitude
    lon: f32,
}

/// Sets up the display for the city, how the name and coordinates should look when printed
impl Display for City {
    // `f` is a buffer, this method must write the formatted string into it
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };   /// Inserting the latitude
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };   /// Inserting the longitude

        // `write!` is like `format!`, but it will write the formatted string into a buffer (the first argument)
        write!(f, "{}: {:.3}°{} {:.3}°{}",
               self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

/// Debug method when displaying the color, and the data types of each color
#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Main method - tests the above methods and prints the specified info below about three different cities
fn main() {
    /// For Loop - iterates through each city and their coordinates
    for city in [
        City { name: "Glassboro", lat: 39.702892, lon: -75.111839 },     /// Displays info for Glassboro with its coordinates
        City { name: "Mullica Hill", lat: 39.73928, lon: -75.224072 },   /// Displays info for Mullica Hill with its coordinates
        City { name: "Swedesboro", lat: 39.747616, lon: -75.310463 },    /// Displays info for Swedesboro with its coordinates
    ].iter() {
        println!("{}", *city);   /// Prints all three of the cities and their info above
    }

    /// For Loop - iterates through each color
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        // Hint : Fix the code so you can print it using {}
        /// The print statement below was updated so that instead of printing color* and getting an output that includes "color",
        /// the output rather manually prints red, blue and green within the print statement so that the colors are displayed without "color" printed before it
        println!("Red: {}, Green: {}, Blue: {}", color.red, color.green, color.blue);
    }
}
