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
Set wallpaper from your command line!

USAGE:
    wlp [OPTIONS] [path]

ARGS:
    <path>    Image path!

OPTIONS:
    -e, --ignore         Ignore file extensions!
    -h, --help           Print help information
    -m, --mode <mode>    What mode to run the program in [possible values: center, crop, fit, span,
                         stretch, tile]
    -r, --random         
```

Example
```shell
wlp -r -m center
```