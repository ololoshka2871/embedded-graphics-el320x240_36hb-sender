# Embeded-graphics for EL320x240_36HB sender for Desktop PC

## Dependencies
* SDL2
* [embedded-graphics-simulator](https://github.com/embedded-graphics/simulator)

### Build
See [this](https://github.com/embedded-graphics/simulator#setup) section of embedded-graphics-simulator README.
You need to install SDL2 in windows with development headers.

### Run
```cmd
cargo run --release -- COMx
```

where COMx is your serial port name.