# Introduction

This is a software rasterizing renderer written in Rust for educational purposes.

The basic framework is based on the guide hosted on ssloy/tinyrenderer.

# Compilation and Execution

Run the program by typing the following inside the root repo directory:

```
cargo run mirai_miku/mirai_miku.obj mirai_miku/combined_image_0_6124682088.png 0 -0.7
```

This runs an unoptimzied debug build, so the render may take some time. 

The render output is a file named `render.png`.

The output resolution may be modified in `main.rs`.
