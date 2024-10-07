# Rusty-Keylog

## Usage
- When you start running the program, it will begin detecting and logging every single action!
- To stop the program, press the F4 key.
- The captured events will be saved in the `keylog` folder as a file named **"output.txt"**.

## Installation

### Using Cargo (Recommended)

**To install this program locally, you need Cargo or a similar Rust package manager installed on your system.**

1. Clone the repository:
    ```
    git clone https://github.com/quotequack/rustyKeylog
    cd rustyKeylog
    ```
2. Compile the program with:
    ```
    cargo build --release
    ```
    After compiling, go to the target/release directory to find the executable (.exe if on Windows).
3. Or run it directly without building:
    ```
    cargo run --release
    ```
### Using docker
If you prefer using Docker to run the program without installing Rust or Cargo, follow these steps:
1. Build the Docker image:
    ```
    docker build -t rusty-keylog .
    ```
2. Run the Docker container:
    ```
    docker run --rm -it --privileged rusty-keylog
    ```

