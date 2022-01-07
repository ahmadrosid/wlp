# WLP
Set wallpaper from your terminal.

## Installation
Clone repository
```bash
git clone https://github.com/ahmadrosid/wlp.git
```

Install using cargo.
```shell
cargo install --path .
```

## Usage
```shell

USAGE:
    wlp [OPTIONS] [path]

ARGS:
    <path>    Image path!

OPTIONS:
    -h, --help           Print help information
    -m, --mode <mode>    What mode to run the program in [possible values: center, crop, fit, span,
                         stretch, tile]
    -r, --random         

```

Example
```shell
wlp -r -m center
```