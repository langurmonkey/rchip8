# rCHIP8, a CHIP-8 emulator

**rCHIP8** is a [CHIP-8](https://tonisagrista.com/blog/2021/chip8-spec) emulator written in Rust and SDL2.
A writeup about this implementation can be found [in my blog](https://tonisagrista.com/blog/2021/chip8-implementation).

Results of running [this ROM](https://github.com/corax89/chip8-test-rom):

![](test-results.jpg)

The [cavern game](https://github.com/mattmikolay/chip-8/tree/master/cavern):

![](cavern.jpg)

Find additional games and demos in the [CHIP-8 Archive](https://johnearnest.github.io/chip8Archive/) or in [this repository](https://github.com/dmatlack/chip8/tree/master/roms)

## Building

To compile for release, just do:

```bash
cargo build --release
```

## Running

You can go ahead and run the executable created in the building step, which is under `target/release/rchip8`:

```bash
target/release/rchip8 [ROM_FILE]
```

Or you can just run it with cargo, with the `--release` target enabled:

```bash
cargo run --release -- [ROM_FILE]
```

### Display scaling

This implementation supports an integer display scale factor to make the display larger. Use it with `-s [FACTOR]`.

### Colors

You can specify the foreground color as a hex string with `-c` or `--fgcol`.

You can specify the background color as a hex string with `-b` or `--bgcol`.

For example:

```bash
rchip -c ABABAB -b 030303
```

### Debug

Enable debug mode with `-d`. In this mode, the program halts before every instruction and prints the instruction itself, the decoded operation, the value of each register and the value of the index I.

### Speed

You can change the emulation speed in instructions per second with `-i [IPS]`. The default value is 1000.

### CLI arguments

Here are the available arguments:

```bash
R-CHIP-8 0.1.0
Toni Sagrsità Sellés <me@tonisagrista.com>
CHIP-8 emulator

USAGE:
    rchip8 [FLAGS] [OPTIONS] <input>

FLAGS:
    -d, --debug      Run in debug mode. Pauses after each instruction, prints info to stdout.
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b, --bgcol <bgcol>    Background (off) color as a hex code, defaults to 101020
    -c, --fgcol <fgcol>    Foreground (on) color as a hex code, defaults to ABAECB
    -i, --ips <ips>        Emulation speed in instructions per second, defaults to 1000
    -s, --scale <scale>    Integer display scaling, defaults to 10 (for 640x320 upscaled resolution)

ARGS:
    <input>    ROM file to load and run.
```
