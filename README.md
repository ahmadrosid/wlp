# WLP
Set wallpaper from your terminal. It works on Linux, Mac and Windows.

## Installation
Manual
```bash
git clone https://github.com/ahmadrosid/wlp.git
cargo install --path wlp/
```

Install using cargo.
```shell
cargo install wlp
```

## Usage
```shell
USAGE:
    wlp [OPTIONS] [path] [SUBCOMMAND]

ARGS:
    <path>    Image path

OPTIONS:
    -e, --ignore         Ignore file extensions
    -h, --help           Print help information
    -m, --mode <mode>    Wallpaper mode option [possible values: center, crop, fit, span, stretch,
                         tile]
    -r, --random         Set random wallpaper from unsplash

SUBCOMMANDS:
    help    Print this message or the help of the given subcommand(s)
    url     Download wallpaper from url
```

Example
```shell
wlp -r -m center
```