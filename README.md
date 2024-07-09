# Flamers 🔥

![[./demo.mp4]]

`flamers` is a terminal program designed for fire simulation, created for 
entertainment purposes. You can customize the appearance of your fire 
simulation by utilizing the following flags and keys:

## Usage

```
flamers [OPTIONS]
```

## Options

- `-z, --zoom <ZOOM>`: Set zoom level. Default: `0.28`
- `-s, --scale <SCALE>`: Set fire simulation scale. Default: `3.0`
- `-d, --detail <DETAIL>`: Set fire simulation detail level. Default: `3`
- `-p, --speed <SPEED>`: Set fire simulation speed. Default: `0.0006`
- `-o, --offset <OFFSET>`: Set vertical offset. Default: `0.0`
- `-f, --fps <FPS>`: Set maximum frames per second. Default: `60`
- `-g, --gradient <GRADIENT>`: Set flame gradient. Default: `#FFFF64;#FFBE1E;#FF9600;#FF5000;#B45000;#503C28;#28281E;#000000`
- `-h, --help`: Print help

## Values

-  `ZOOM`: Integer > 0
-  `SCALE`: Float >= 0
-  `DETAIL`: Integer > 0
-  `SPEED`: Float >= 0
-  `OFFSET`: Float
-  `FPS`: Integer > 0
-  `GRADIENT`: Semicolon-separated string of hex colors in the format `#RRGGBB`.
  Example values:
  -  Orange: `#FFFF64;#FFBE1E;#FF9600;#FF5000;#B45000;#503C28;#28281E;#000000`
  -  Blue:   `#64FFFF;#1EBEFF;#0096FF;#0050FF;#0050B4;#283C50;#1E2828;#000000`
  -  Green:  `#78FF96;#1EFF5A;#00C850;#00B446;#00963C;#285028;#1E281E;#000000`

## Available keymaps

- `q`: Quit
- `+`: Zoom in
- `-`: Zoom out
- `s`: Decrease scale
- `S`: Increase scale
- `d`: Increase detail level
- `D`: Decrease detail level
- `e`: Increase speed
- `E`: Decrease speed
- `<Up>, k`: Decrease vertical offset (Scroll up)
- `<Down>, j`: Increase vertical offset (Scroll down)

## Installation

Using cargo:

```bash
cargo install --git https://github.com/danilax999/flamers.git
```

Using nix flakes

```bash
nix run https://github.com/danilax999/flamers.git
```
