# SPFresh Docker Project

This project provides a Dockerized environment for building and running the SPFresh application. It automates the process of cloning the SPFresh repository, initializing its submodules, installing dependencies, and compiling the necessary libraries.

## Project Structure

- **Dockerfile**: Contains instructions to build the Docker image.
- **docker-compose.yml**: Defines services, networks, and volumes for the application.
- **.dockerignore**: Lists files and directories to ignore when building the image.
- **Makefile**: Contains directives for building the project and running tasks.
- **scripts/**: Directory containing various scripts for cloning, installing dependencies, and building libraries.
  - `clone-and-init.sh`: Clones the SPFresh repository and initializes submodules.
  - `install-deps.sh`: Installs necessary dependencies.
  - `build-spdk.sh`: Compiles the SPDK library.
  - `build-isal.sh`: Compiles the isal-l_crypto library.
  - `build-rocksdb.sh`: Builds the modified RocksDB library.
  - `build-spfresh.sh`: Compiles the SPFresh project.
  - `entrypoint.sh`: Entry point for the Docker container.
- **configs/**: Contains configuration scripts.
  - `gcc-setup.sh`: Sets up a higher version of GCC for compilation.
- **README.md**: Documentation for the project.

## Setup Instructions

1. **Clone the Repository**: The Dockerfile will automatically clone the SPFresh repository and initialize its submodules.
2. **Build the Docker Image**: Use the following command to build the Docker image:
   ```
   docker build -t spfresh .
   ```
3. **Run the Application**: Use Docker Compose to start the application:
   ```
   docker-compose up
   ```

## Usage

After the application is running, you can interact with it as specified in the SPFresh documentation. Ensure that all dependencies are correctly installed and that the libraries are built successfully.

## Contributing

Contributions are welcome! Please submit a pull request or open an issue for any enhancements or bug fixes.

## License

This project is licensed under the MIT License. See the LICENSE file for details.