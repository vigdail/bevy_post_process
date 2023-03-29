# Chromatic aberration using post process pass

This is an adaptation
of [this](https://github.com/bevyengine/bevy/blob/main/examples/shader/post_process_pass.rs) code from main
branch to bevy v0.10.0

## Build and run

You can run the program using
```shell
cargo run --release
```

To generate svg image of render graph run (`graphviz` should be installed)
```shell
cargo run --release -- --graph | dot -Tsvg > render_graph.svg
```