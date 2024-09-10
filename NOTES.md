# Setup notes for embedded Rust

## Bare-metal Rust

Available Rust platforms are listed on [The rustc book, chapter Platform support](https://doc.rust-lang.org/nightly/rustc/platform-support.html)


Rust/LLVM Target Triple: 
```<arch><sub>-<vendor>-<sys>-<env>
```
arch = (x86_64, i386, arm, etc)
sub = (v6, v7m, etc)
vendor is optional
sys = (none, linux, win32, etc)
env = (eabi, gnu, elf, etc)

To find ou the right one, we look at the uController architecture of the board

### Micro:bit v2
The micro-controller is a nRF52833, a Cortex M4 with FPU, 
The Cortex M4 architecture is ***Armv7E-M***. A look up on the available platforms gives either `thumbv7em-none-eabi` or `thumbv7em-none-eabihf`

```bash
rustup target add thumbv7em-none-eabihf
rustup show
```
### The run-time
This is the run time library for Rust, which handle the start up code, interrupts, enable FPU, etc.
```
cargo add cortex-m-rt 
```

It needs a `memory.x` file at the op level to establish the Flash and RAM areas (at minumum)
It also needs some compile flags, which we can specifiy in a `.cargo/config.toml` file for the given target `thumbv7em-none-eabihf`

### The panic handler

Either we can write our oun or just use a simple panic handler that des nothing using a library

```bash
cargo add panic_halt
```

### LLVM tools for cross compilation

These help to inpect generated artifacts through cargo commands

```bash
rustup component add llvm-tools
cargo install cargo-binutils
```
Example:
```bash
cargo size -- -Ax

   Finished 
             rustymicrobit  :
section                size         addr
.vector_table         0x400          0x0
.text                0x218c        0x400
.rodata               0xa20       0x258c
.data                     0   0x20000000
.gnu.sgstubs              0       0x2fc0
.bss                  0x440   0x20000000
.uninit                   0   0x20000440
.debug_abbrev        0x20f4          0x0
.debug_info         0x2b712          0x0
.debug_aranges       0x19f0          0x0
.debug_str          0x42d34          0x0
.comment               0x40          0x0
.ARM.attributes        0x3a          0x0
.debug_frame         0x53f8          0x0
.debug_line         0x2235d          0x0
.debug_ranges       0x1b048          0x0
.debug_loc            0x207          0x0
.debug_pubnames       0x2be          0x0
.debug_pubtypes        0x47          0x0
Total               0xb8039
```

### Flashing the microcontroller

We also need some udev rules:

File 99-microbit.rules:
```
SUBSYSTEM=="usb", ATTR{idVendor}=="0d28", ATTR{idProduct}=="0204", MODE:="666"
SUBSYSTEM=="hidraw", MODE:="666"
```
and reload the rules
```bash
sudo udevadm control --reload-rules && sudo udevadm trigger
```

```bash
cargo install probe-rs-tools 
```

To flash the chip, use `cargo embed`. 

The defaults for cargo embed are contained in an `Embed.toml` file.
Example:

```toml
[default.general]
chip = "nrf52833_xxAA"

[default.reset]
halt_afterwards = false

[default.rtt]
enabled = true

[default.gdb]
enabled = false
```
### Debugging using Real Time Transfer (RTT) interface 

The `rtt-target` crate defines a `rprintln!` macro which needs a platform-specific `critical-section` implementation
This implementation is found in the `cortex-m` library with added feature `critical-section-single-core`

```bash
cargo add rtt-target
cargo add cortex-m --features critical-section-single-core
```

## Other Useful Crates

* *embassŷ* - Async I/O for embedded Rust
* *postcard* - M2M tools and RPC-style protocol for embedded Rust
* *heapless* - Heapless data sructures like Vec or String
