# Quick Release

A simple CLI tool to quickly release a new version of a LiterateInk library implementation.

Of course, this is only useful for LiterateInk libraries and is only meant to be used by the maintainers of the libraries.

## Motivation

In some implementations, such as `js`, we had custom tools to do this but it was only bound to that specific implementation. For example, `release-it` for the `js` implementation. But, I wanted a tool that could be used in any implementation without doing any extra config, work or setup.

So, we created this tool to automate the process without any configuration or setup. It's a simple CLI tool that can be used in any of our library repositories to quickly release a new version.

## Installation

You can only install it by building it from source.
Make sure you have `cargo` and `rust` installed.

```bash
cargo build --release
sudo mv ./target/release/quick-release /usr/local/bin/ink-quick-release

# Here, the command is `ink-quick-release`
# In my shell profile, I like to add an alias:
#
# alias ink-qr="ink-quick-release"
```

Note that the `mv` step only works on Linux and macOS. You might have to manually copy the binary to the correct path on Windows or even add the path to the binary to your `PATH` environment variable.

## Usage

Be in a LiterateInk repository, in an implementation branch (such as `js` or `kotlin`) and you can directly run the command.

```bash
ink-quick-release
```

It'll ask you for the type of bump you want for the new version, and then it'll create a new commit and tag and push it to the implementation branch.

It'll also redirect you to the GitHub page to create a new release with the tag, release name and the release notes - generated using a `git log`.


## Uninstallation

You can simply remove the binary and it'll be gone.
It doesn't create any other files or directories, so you don't have to worry about that.

```bash
sudo rm /usr/local/bin/ink-quick-release
```

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.
