# bluepill-template
Cargo generate template for Rust embedded projects based on the stm32f103 blue pill board.

## Usage
Install cargo-generate if you don't already have it;
```cargo install cargo-generate```

Generate your project;
```cargo generate --git https://github.com/mendelt/bluepill-template```

Configure your project debugger
Edit .cargo/config, set runner to the executable you use as your debugger.

for example:
```runner = "arm-none-eabi-gdb -q -x debug.gdb"```
or
```runner = "gdb-multiarch -q -x debug.gdb"```

Start openocd in a separate terminal window by running `openocd` from the root or your project.
This will make sure it picks up the openocd.cfg file.

Start the project:
```cargo run```
