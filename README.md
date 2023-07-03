# raytracer [![Rust](https://github.com/joshaustintech/raytracer/actions/workflows/rust.yml/badge.svg)](https://github.com/joshaustintech/raytracer/actions/workflows/rust.yml)

Simple raytracer written in Rust

## Introduction

I'm following the online course *[Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html)*
but am using Rust instead of the example C++ code.

## Progress

### Reflective Material
![Reflective material, two spheres](render_1688285882.jpg)
![Reflective material, grid of spheres](render_1688344777.jpg)

### Shading ("Diffuse Materials")
![Diffuse Materials](render_1688111057.jpg)

### Edge Smoothing ("Antialiasing")
![Edge Smoothing](render_1688099584.jpg)

### Simple "World"
![Colored Sphere](render_1688022712.jpg)

### Colored Sphere
![Colored Sphere](render_1688005594.jpg)

### Initial Render
![Initial Render](render_1687990097.jpg)

## Usage
### Build
```bash
cargo build
```
### Test
```bash
cargo test
```
### Run
```bash
cargo run
```
