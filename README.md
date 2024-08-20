# ASCII Image Evolution

This project uses an evolutionary algorithm to generate an ASCII art representation of an input image. The goal is to approximate the original image by evolving a population of ASCII characters with varying sizes and colors. The program utilizes the `imageproc` library for drawing images and the `img_hash` library to calculate the fitness function of the algorithm, which measures how closely the generated ASCII image matches the original.

## Features

- **Evolutionary Algorithm**: The algorithm evolves a population of ASCII characters, optimizing their placement, size, and color to match the input image.
- **Fitness Function**: The fitness of each individual (image) is determined using the `img_hash` library, which compares the generated image to the original input.
- **Image Drawing**: Uses the `imageproc` library for drawing ASCII characters onto the canvas.

## Installation

To get started with this project, you'll need to have Rust installed. Then, you can clone the repository and build the project:

```bash
git clone <repository-url>
cd ascii-image-evolution
cargo build --release
```

## Usage

After building the project, you can run it by providing an input image (Save it in directory and link it in the `main.rs` file.):

```bash
cargo run --release -- input_image.png
```

The program will generate an ASCII representation of the input image and output it as an image file.

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, please feel free to open an issue or submit a pull request.
