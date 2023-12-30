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

Then set the vars (these are from an ARM machine):

```shell
export JQ_LIB_DIR=/lib/aarch64-linux-gnu
export ONIG_LIB_DIR=/lib/aarch64-linux-gnu
```

## Building

With these set to the appropriate values for your system, running `make build` should result in a usable `./bin/mdsplode`.
