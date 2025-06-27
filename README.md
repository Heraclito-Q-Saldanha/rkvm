# rkvm
[![rkvm](https://img.shields.io/aur/version/rkvm)](https://aur.archlinux.org/packages/rkvm)

rkvm is a tool for sharing keyboard and mouse across multiple Linux machines.
It is based on a client/server architecture, where server is the machine controlling mouse and keyboard and relays events (mouse move, key presses, ...) to clients.

Switching between different clients is done by a configurable keyboard shortcut.

## Features
- Display server agnostic (in fact, it doesn't require a display server at all)
- Low overhead

## Requirements
- The uinput Linux kernel module, enabled by default in most distros. You can confirm that it's enabled in your distro by checking that `/dev/uinput` exists.
- libevdev development files (`sudo apt install libevdev-dev` on Debian/Ubuntu)
- Clang/LLVM (`sudo apt install clang` on Debian/Ubuntu)

## Manual installation
If you can, it is strongly recommended to use the [AUR package](https://aur.archlinux.org/packages/rkvm) to install rkvm.  
Note that the master branch can contain untested and breaking changes - for regular use, it is recommended to pick the latest [release](https://github.com/htrefil/rkvm/releases) instead.

```sh
cargo build --release
cp target/release/rkvm-client /usr/bin/
cp target/release/rkvm-server /usr/bin/
cp systemd/rkvm-client.service /usr/lib/systemd/system/
cp systemd/rkvm-server.service /usr/lib/systemd/system/
```

## Configuration
After installation:
- Create a config if you haven't done so already.  
  Server:  
  ```sh
  cp /usr/share/rkvm/examples/server.toml /etc/rkvm/server.toml
  ```
  Client:
  ```sh
  cp /usr/share/rkvm/examples/client.toml /etc/rkvm/client.toml
  ```
  Do not edit the example configs, they will be overwritten by your package manager.
- **Change the password** and optionally reconfigure the network listen address and key bindings for switching clients  
- Since rkvm-server grabs all input, i's a good idea to do a test run first to make sure you won't end up
  being unable to user your keyboard and/or mouse because your display server is not properly configured to receive input from rkvm.

  Run the following command to start rkvm-server for 15 seconds to test that your keyboard, mouse, etc. works properly:
  ```sh
  rkvm-server /etc/rkvm/server.toml --shutdown-after 15
  ```

- Enable and start the systemd service.  
  Server:
  ```sh
  systemctl enable rkvm-server
  systemctl start rkvm-server
  ```
  Client:
  ```sh
  systemctl enable rkvm-client
  systemctl start rkvm-client
  ```

## Why rkvm and not Barrier/Synergy?
The author of this program had a lot of problems with said programs, namely his keyboard layout (Czech) not being supported properly, which stems from the fact that the programs send characters which it then attempts to translate back into keycodes. rkvm takes a different approach to solving this problem and doesn't assume anything about your keyboard layout -- it sends raw keycodes only.

Additionally, rkvm doesn't even know or care about X, Wayland or any display server that might be in use, because it uses the uinput API with libevdev to read and generate input events.

Regardless, if you want a working and stable solution for crossplatform keyboard and mouse sharing, you should probably use either of the above mentioned programs for the time being.

## Limitations
- Linux only

## Project structure
- `rkvm-server` - server application code
- `rkvm-client` - client application code
- `rkvm-input` - handles reading from and writing to input devices
- `rkvm-net` - network protocol encoding and decoding

[Bincode](https://github.com/servo/bincode) is used for encoding of messages on the network and [Tokio](https://tokio.rs) as an asynchronous runtime.

## Contributions
All contributions, that includes both PRs and issues, are very welcome.

## Donations
If you find rkvm useful, you can donate to the original author and maintainer using [Ko-fi](https://ko-fi.com/htrefil).

## License
[MIT](LICENSE)
