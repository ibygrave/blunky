Blink an LED on Arduino UNO
===========================

From a copy of the avr-hal-template.
Now uses only the [avr-device crate](https://crates.io/crates/avr-device).

`cargo run` tries to load the target on an attached arduino.
If that fails it runs the target with `qemu-system-avr`
and connects `avr-gdb` to it.

## License

 - MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
