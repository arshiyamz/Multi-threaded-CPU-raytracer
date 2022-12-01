# Multi-threaded CPU raytracer created in rust

## Introduction
*This project was made with the help of [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html) for the raytracing concepts.*
This is a simple raytracer with support for Lambertian, metalic and transparent materials, with accurate reflections and refractions.
The only shape supported at this point is a sphere.
There is also support for a camera with variable position, FOV and depth of field.
There is no denoising or super-sampling and aliasing is addressed using MSAA, sampling multiple rays with small random offsets for each pixel.
Although this raytracer is multi-threaded, it is not intended to be a real-time raytracer unless you have a CPU with a high thread count and an extremely high throughput :)

## Implementation Details
The project is written entirely in standard rust without the use of any external packages (also known as crates in the rust context).
I have implemented any required functionality not included in standard rust myself. This includes a seeded deterministic random number generator and an extensive vector math library.
The libraries I created have unit tests to ensure their correctness.
The multi-threading is accomplished by dividing the framebuffer (also known as render target) into slices depending on the number of available threads by the system to avoid data races without needing to lock resources.
Each thread has its own random number generator, and the generated numbers have sufficient entropy such that it is impossible to distinguish the slices in the final render image.


## Usage
To run this renderer, you need to setup a world in the main() method of src/main.rs. You can look at make_simple_scene() and make_random_scene() (both in src/main.rs) as examples.
You can also adjust the render width and height, the number of rays per pixel (i.e. the number of samples) and the number of child rays through the constants and the top of main.rs.
You can adjust the camera FOV, depth of field, aperture, location and orientation in the main() method in main.rs.
After you have adjusted the settings and created your desired world, you can run the render by calling `cargo run` in the project root if you have rust installed.
The result will be put in out/output.ppm and it is in the PPM format.
You can view this output.ppm file using your favorite PPM viewer or using [_this one_](https://raytracing.github.io/books/RayTracingInOneWeekend.html).

## Example
Here is an example render made using this raytracer with the make_random_scene() function:
![example-render](https://user-images.githubusercontent.com/77579116/204995436-6edd488b-181e-485c-b753-a3e1ebd2cfbd.png)
The ppm file has been translated into the PNG format for easier viewing. If you are curious about the output.ppm file of this render you can find it in the example folder.
The glass balls might look weird but thats because we don't often see glass balls in real life. The renders should be physically accurate :)
