# nu_plugin_msgpack

A plugin for [nushell](https://nushell.sh/) that provides the commands
`from msgpack` and `to msgpack` to convert between Nu types and [MsgPack](https://msgpack.org/),
which is a JSON-like binary serialization format.

![ls-to-msgpack.gif](https://github.com/hulthe/nu_plugin_msgpack/blob/master/showcase/ls-to-msgpack.gif)

## Installation

```sh
# download and install the plugin
cargo install --git https://github.com/hulthe/nu_plugin_msgpack.git

# register the plugin with nu, you should put this in your `config.nu`
register ~/.cargo/bin/nu_plugin_msgpack
```

## Quirks

There's not a 1 to 1 mapping between Nu-types and MsgPack-types.
Here are some conversions that `to msgpack` does which might trip you up:

- `filesize` becomes an integer of bytes. This is consistent with `to json`.
- `duration` becomes an integer of nanoseconds. This is consistent with `to json`.
- `range` becomes an array. This is consistent with `to json`.
- The following Nu types become nil: `block`, `closure`, `error`, `cell_path`, and `match_pattern`.

Additionally Nu `date`s are converted to the MsgPack [timestamp extension type](https://github.com/msgpack/msgpack/blob/master/spec.md#timestamp-extension-type).
This differs from `to json` which converts dates to strings.
