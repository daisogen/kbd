# kbd
`kbd` is a keyboard abstraction for Daisōgen.
- It separates hardware keyboard from software functions
- It handles layouts, powered by [kbd2csv](https://github.com/jlxip/kbd2csv).

It must be started before any keyboard driver, and exposes a pointer named `kbd_buffer_keycode`. Keyboard drivers must call this function passing a keycode as an argument.

The rest of the public API is pending.

In order to save space, only `us` and `es` are included by default. If you want to add one, add its name to `steal.txt`: it will be downloaded from [here](https://github.com/legionus/kbd), converted using [kbd2csv](https://github.com/jlxip/kbd2csv), and bundled in the binary.
