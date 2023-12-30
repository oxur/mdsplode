# Supporting `jq` Queries

*(for filtering results)*

This tool filters using `jq` query strings and this requires that you let `mdsplode` know where your `jq` and other dependent libs are installed.

## macos/Darwin

If you're on a mac and `jq` was installed with homebrew, you can get the lib directories with the following:

```shell
brew list jq
```

```shell
/opt/homebrew/Cellar/jq/1.6/bin/jq
/opt/homebrew/Cellar/jq/1.6/include/ (2 files)
/opt/homebrew/Cellar/jq/1.6/lib/libjq.1.dylib
/opt/homebrew/Cellar/jq/1.6/lib/ (2 other files)
/opt/homebrew/Cellar/jq/1.6/share/doc/ (4 files)
/opt/homebrew/Cellar/jq/1.6/share/man/man1/jq.1
```

```shell
brew list oniguruma
```

```shell
/opt/homebrew/Cellar/oniguruma/6.9.7.1/bin/onig-config
/opt/homebrew/Cellar/oniguruma/6.9.7.1/include/ (2 files)
/opt/homebrew/Cellar/oniguruma/6.9.7.1/lib/libonig.5.dylib
/opt/homebrew/Cellar/oniguruma/6.9.7.1/lib/pkgconfig/oniguruma.pc
/opt/homebrew/Cellar/oniguruma/6.9.7.1/lib/ (2 other files)
```

Use these to set the `JQ_LIB_DIR` environment variable; in this case:

```shell
export JQ_LIB_DIR=/opt/homebrew/Cellar/jq/1.6/lib
export ONIG_LIB_DIR=/opt/homebrew/Cellar/oniguruma/6.9.7.1/lib
```

## Debian/Ubuntu

If you're on a Debian-based system, it's a little simpler, though the non-standard environment variables still need to be set. Install the required packages:

```shell
sudo apt-get install jq libjq-dev libonig-dev
```

You can get the location of the installed library files with the commands:

```shell
ldd $(which jq)
```

```text
linux-vdso.so.1 (0x0000007f7fb97000)
libjq.so.1 => /lib/aarch64-linux-gnu/libjq.so.1 (0x0000007f7fae2000)
libc.so.6 => /lib/aarch64-linux-gnu/libc.so.6 (0x0000007f7f96d000)
/lib/ld-linux-aarch64.so.1 (0x0000007f7fb67000)
libm.so.6 => /lib/aarch64-linux-gnu/libm.so.6 (0x0000007f7f8c2000)
libonig.so.5 => /lib/aarch64-linux-gnu/libonig.so.5 (0x0000007f7f823000)
```

As you can see, this example is from an ARM-based machine.

Then set the vars:

```shell
export JQ_LIB_DIR=/lib/aarch64-linux-gnu
export ONIG_LIB_DIR=/lib/aarch64-linux-gnu
```

## Building

With these set to the appropriate values for your system, running `make build` should result in a usable `./bin/mdsplode`.
