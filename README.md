# RRTK Stream Builder
**Code generation from visual nodes for the [RRTK](https://crates.io/crates/rrtk) stream system.**

This program is still pretty early in development and there are notable important features missing. The most important of these are the inability to save workflows and the inability to name nodes.
## License: BSD 3-Clause
This basically means that you can do whatever you want as long as you give me attribution and you don't remove the license notices or use my name to endorse stuff I don't. Read the actual license for details though.

**RRTK Stream Builder was previously licensed under LGPL. Versions 0.1.1 and earlier have been retroactively dual licensed under LGPL-3.0-only OR BSD-3-Clause. Versions after 0.1.1 are just BSD-3-Clause.** This transition does not remove any freedoms, and the LGPL for old versions is maintained solely due to its irrevocability. It is entirely redundant freedoms-wise.
## Installation
Installation is pretty much like any other gtk4-rs app. These are installation instructions for Linux. See the [gtk-rs documentation](https://gtk-rs.org/gtk4-rs/stable/latest/book/installation.html) for more details and other platforms. You will also need the Rust toolchain already installed. If you have not yet done this, see the [Rust installation instructions](https://www.rust-lang.org/tools/install).
### Fedora
```
sudo dnf install gcc gtk4-devel
cargo install rrtk_stream_builder
```
### Debian
```
sudo apt install libgtk-4-dev build-essential
cargo install rrtk_stream_builder
```
### Arch
```
sudo pacman -S gtk4 base-devel
cargo install rrtk_stream_builder
```
### Adding the Cargo binary directory to `PATH` [All Linux]
This section assumes that you are using Bash. If this is not the case, check the documentation for your shell to find out how to do this.

If, after running `cargo install rrtk_stream_builder`, you see a warning like this:
```
warning: be sure to add `/home/username/.cargo/bin` to your PATH to be able to run the installed binaries
```
you can run this to fix it:
```
echo "export PATH=~/.cargo/bin:$PATH" >> ~/.bashrc
source ~/.bashrc
```
`~` is an alias for `/home/username`, or your home directory. If you want to write it out fully like Cargo does in its warning, that will also work.
### Running
As with most binary crates, just run the crate name as a terminal command.
```
rrtk_stream_builder
```
## Changes
### 0.1.0
Initial release.
### 0.1.1
Add support for RRTK 0.4.
### 0.2.0
Stream Builder 0.2.0 is almost entirely rewritten. There are a few reasons for this, but essentially, as you can see if you read the earlier part of the Git log, Stream Builder began as a test of GTK 4 and drag-and-drop with Cairo, and instead of developing into a fuller application, it effectively became a very overgrown test, having poor structure and containing lots of repeated code. It was very difficult to maintain and only received a single update after the initial release (before being rewritten). I've also, more simply, learned more about Rust and some useful features it has, particularly closures and macros. Separating Cairo drag-and-drop into its own crate, [`cairodrag`](https://crates.io/crates/cairodrag), has also greatly improved the structure. There are however, in spite of the rewrite, relatively few changes that the end user would notice:
- Support was added for RRTK 0.5 and 0.6 and maintained for 0.3 and 0.4.
- Deleting nodes is now done with middle click rather than right click.
- It is now possible to scroll by dragging the background.
- Nodes are now organized by module rather than by version added.
