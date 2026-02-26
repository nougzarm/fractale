
# 🦀 fractale: Mandelbrot Generator

A fast, multi-threaded Mandelbrot set generator written in **Rust**. This CLI tool leverages the power of parallel computing to render beautiful fractals at high resolutions and deep zoom levels.



## ✨ Features

* **⚡ High Performance:** Written in Rust for near-native speeds.
* **🧵 Multi-threaded:** Uses the `Rayon` library to distribute pixel calculations across all available CPU cores.
* **🔍 Deep Zoom:** Explore the infinite complexity of the Mandelbrot set with adjustable coordinates and zoom levels.
* **🛠️ CLI Interface:** Powered by `Clap` for a clean and intuitive command-line experience.

---

## 🚀 Quick Start

### Prerequisites
Ensure you have the Rust toolchain installed. If not, get it at [rustup.rs](https://rustup.rs/).

### Installation
Clone the repository and build the project:

```bash
git clone [https://github.com/nougzarm/fractale.git](https://github.com/nougzarm/fractale.git)
cd fractale
cargo build --release

```

### Usage

Generate the classic Mandelbrot set:

```bash
cargo run --release -- -W 1000 -H 1000 --output gallery.png

```

---

## 🎨 Discovery Tour

Try these coordinates to find the most beautiful spots in the fractal (remember to use `--release` for these!):

| Location | Command |
| --- | --- |
| **Seahorse Valley** | `cargo run --release -- -x=-0.768 -y 0.145 -z 500 -i 500` |
| **Triple Satellite** | `cargo run --release -- -x=-0.160 -y 1.033 -z 2000 -i 1000` |
| **The Great Heart** | `cargo run --release -- -x=-0.75 -y 0 -z 1 -i 255` |

---

## 🛠️ Configuration Options

| Flag | Long Flag | Description | Default |
| --- | --- | --- | --- |
| `-W` | `--width` | Width of the output image | 800 |
| `-H` | `--height` | Height of the output image | 800 |
| `-x` | `--center-x` | Real axis center coordinate | -0.75 |
| `-y` | `--center-y` | Imaginary axis center coordinate | 0.0 |
| `-z` | `--zoom` | Zoom level (higher is deeper) | 1.0 |
| `-i` | `--max-iter` | Precision (iteration depth) | 255 |
| `-o` | `--output` | Output filename (.png) | mandelbrot.png |

> **Note:** If you encounter issues with negative values (e.g., `-x -0.75`), use the `=` syntax: `--center-x=-0.75`.

---

## 🧠 How it Works

The program iterates the function $z_{n+1} = z_n^2 + c$ for every pixel. If the sequence stays bounded, the point belongs to the set.

1. **Coordinate Mapping:** Converts screen pixels $(u, v)$ to the complex plane $(x, y)$ using the specified zoom and center.
2. **Parallel Computation:** Using `Rayon`, the image buffer is split into chunks, allowing each CPU core to work on a slice of the image simultaneously.
3. **Colorization:** Maps the escape velocity (iteration count) to a custom RGB gradient.

---

Generated with ❤️ using Rust.
