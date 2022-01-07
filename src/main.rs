use clap::{arg, App, ArgEnum, PossibleValue};
use std::path::Path;
use wallpaper;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum ModeOption {
    Center,
    Crop,
    Fit,
    Span,
    Stretch,
    Tile,
}

impl ModeOption {
    pub fn possible_values() -> impl Iterator<Item = PossibleValue<'static>> {
        ModeOption::value_variants()
            .iter()
            .filter_map(ArgEnum::to_possible_value)
    }
}

impl std::fmt::Display for ModeOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

impl std::str::FromStr for ModeOption {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for variant in Self::value_variants() {
            if variant.to_possible_value().unwrap().matches(s, false) {
                return Ok(*variant);
            }
        }
        Err(format!("Invalid variant: {}", s))
    }
}

fn main() {
    let matches = App::new("wlp")
        .about("Set wallpaper from your command line!")
        .arg(arg!([path] "Image path!"))
        .arg(arg!(-r - -random))
        .arg(
            arg!(-m - -mode)
                .help("What mode to run the program in")
                .possible_values(ModeOption::possible_values()),
        )
        .get_matches();

    if let Some(path) = matches.value_of("path") {
        let image_path = Path::new(path);
        if let Some(ext) = image_path.extension() {
            println!("Got file extensions!");
            match ext.to_str().unwrap() {
                "png" | "jpg" => set_from_path(path),
                _ => {
                    println!("Unsupported image extensions {:?}", ext)
                }
            }
        } else {
            println!("Invalid image path {}", path);
            std::process::exit(0x001)
        }
    }

    if matches.is_present("random") {
        let url = "https://source.unsplash.com/random";
        println!("Fetching random image from {}!", url);
        match wallpaper::set_from_url(url) {
            Ok(_) => println!("Done, image path: {}!", wallpaper::get().unwrap()),
            Err(msg) => println!("Error {}", msg),
        }
    }

    match matches
        .value_of_t("mode")
        .expect("'mode' is required and parsing will fail if its missing")
    {
        ModeOption::Center => wallpaper::set_mode(wallpaper::Mode::Center),
        ModeOption::Crop => wallpaper::set_mode(wallpaper::Mode::Crop),
        ModeOption::Fit => wallpaper::set_mode(wallpaper::Mode::Fit),
        ModeOption::Span => wallpaper::set_mode(wallpaper::Mode::Span),
        ModeOption::Stretch => wallpaper::set_mode(wallpaper::Mode::Stretch),
        ModeOption::Tile => wallpaper::set_mode(wallpaper::Mode::Tile),
    }.unwrap()
}

fn set_from_path(path: &str) {
    match wallpaper::set_from_path(path) {
        Ok(_) => println!("Done, image path: {}!", wallpaper::get().unwrap()),
        Err(msg) => println!("Error {}", msg),
    }
}
