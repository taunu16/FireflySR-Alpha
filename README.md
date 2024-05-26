# FireflySR-Alpha 
>Original: https://git.xeondev.com/reversedrooms/FireflySR

A Server emulator for the game [`Honkai: Star Rail`](https://hsr.hoyoverse.com/en-us/)
![screenshot](https://github.com/taunu16/FireflySR-Alpha/blob/master/screenshot.png?raw=true)

## Prerequisites

- [MongoDB](https://www.mongodb.com/try/download/community)

## Installation

### From Source (recommended)

#### Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- [MongoDB](https://www.mongodb.com/try/download/community-edition)
- [Protobuf Compiler (protoc)](https://github.com/protocolbuffers/protobuf/releases/download/v26.1/protoc-26.1-win64.zip). You should manually extract it to somewhere and add `bin` subfolder to the system environment variable `PATH`.

#### Building

#### Using xtasks (recommended)

```sh
git clone https://git.xeondev.com/reversedrooms/FireflySR.git
cd FireflySR
cargo xtask run
```

##### To run it with automatic recompilation when any Rust files are changed

```sh
cargo xtask watch
```

### From Pre-built Binaries

Navigate to the [Releases](https://git.xeondev.com/reversedrooms/FireflySR/releases)
page and download the latest release for your platform.

## Usage

To begin using the server, you need to run three servers: `sdkserver`, `dispatch` and `gameserver`

If you installed from pre-built binaries, navigate to the directory where you downloaded
the binaries and either a) double-click on the following executable names or b)
run the following in a terminal:

```sh
./gameserver
./dispatch
./sdkserver
```

##### Note: the `assets` folder should be in the same directory with the `gameserver`, otherwise it will not boot up.

## Configuration
By default, servers will try to use local mongodb (at 127.0.0.1:27017),
this should work out of box if you installed MongoDB on your machine.
<br>
You can change this in configuration file of specific server.
Currently only sdkserver and gameserver communicate with database, so if you **need** to configure it,
edit sdkserver.json and gameserver.json files.

## Connecting

### For the latest 2.3 Beta patch

If you want to play the latest Beta patch, the `mhypbase.dll` patch for 2.2.51 OS is not usable. You should use a separate proxy like [FireflySR.Tool.Proxy](https://git.xeondev.com/YYHEggEgg/FireflySR.Tool.Proxy) (Prebuilt binary can be downloaded [here](https://git.xeondev.com/YYHEggEgg/FireflySR.Tool.Proxy/releases)).

Also, you need to configure `check_passwords` in `sdkserver.json`. That is to say, if you have used a older version, you should go to the configuration file **in the root directory** and check if it's `false`.

Clients (2.3 Beta v3):

- [CN - 2.2.53](https://autopatchcn.bhsr.com/client/beta/20240517110535_d7lJoh4jYXkVGXI1/StarRail_2.2.53.zip)
- [OS - .2.53](https://autopatchos.starrails.com/client/Beta/20240517111205_PZfNSHVLH509e76v/StarRail_.2.53.zip)

Notice: **CN Package may not have languages other than Chinese, and vice versa**. Be careful when choosing which to download; luckily they are all compatiable with this server.

### For v1 Players

[Get 2.3 beta client v1(.51)](https://autopatchos.starrails.com/client/Beta/20240501125700_dUBAjS7YiX9nF7mJ/StarRail_2.2.51.zip),
replace [mhypbase.dll](https://git.xeondev.com/reversedrooms/FireflySR/raw/branch/master/mhypbase.dll)
file in your game folder, it will redirect game traffic (and disable in-game censorship)

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss
what you would like to change, and why.

## Bug Reports

If you find a bug, please open an issue with as much detail as possible. If you
can, please include steps to reproduce the bug.

Bad issues such as "This doesn't work" will be closed immediately, be _sure_ to
provide exact detailed steps to reproduce your bug. If it's hard to reproduce, try
to explain it and write a reproducer as best as you can.