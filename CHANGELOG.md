# Changes
## 0.1.0
Initial release.
## 0.1.1
Add support for RRTK 0.4.
## 0.2.0
Stream Builder 0.2.0 is almost entirely rewritten. There are a few reasons for this, but essentially, as you can see if you read the earlier part of the Git log, Stream Builder began as a test of GTK 4 and drag-and-drop with Cairo, and instead of developing into a fuller application, it effectively became a very overgrown test, having poor structure and containing lots of repeated code. It was very difficult to maintain and only received a single update after the initial release (before being rewritten). I've also, more simply, learned more about Rust and some useful features it has, particularly closures and macros. Separating Cairo drag-and-drop into its own crate, [`cairodrag`](https://crates.io/crates/cairodrag), has also greatly improved the structure. There are however, in spite of the rewrite, relatively few changes that the end user would notice:
- Support was added for RRTK 0.5 and 0.6 and maintained for 0.3 and 0.4.
- Deleting nodes is now done with middle click rather than right click.
- It is now possible to scroll by dragging the background.
- Nodes are now organized by module rather than by version added.
## 0.2.1-alpha.0
Start implementing file saving and opening through `rrtk_rsb`.
