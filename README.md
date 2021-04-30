# Ray Tracing In One Weekend

![final rendering](https://github.com/christopheslv/ray-tracing-iow/blob/main/assets/13_final_rendering.jpg?raw=true)

Yet another implementation of Peter Shirley's [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html) book, in [Rust](https://www.rust-lang.org).

### Status ###

A very good way to try and learn a new programming language, Rust, however this might not be the best code around.
This is the straight out 1-1 structure and naming choices from the book, there is certainly plenty of room for optimizations by taking advantage of Rust's specificities.

Next steps might be about implementing the following books, with some re-interpretation of the design decisions, and/or some use case specialization.

### Run ###

```sh
# Move to project base directory
cd ray-tracing-iow

# Build Release 
cargo build --release

# Generate ppm image in current folder
./target/release/ray-tracing-iow > image.ppm

# Preview the image (MacOS only)
open image.ppm
```

For higher quality renderings, and longer computation time, try changing the img_width to 800 and samples_per_pixel to 250 in main.rs.

### References ###

Peter Shirley's [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html)

### Code License ###

MIT, see [LICENSE.md](http://github.com/christopheslv//ray-tracing-iow/blob/main/LICENSE.md) for details.