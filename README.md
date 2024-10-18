# Nimbus - File/Folder Upload Tool

Nimbus is a versatile command-line interface (CLI) tool designed to simplify the process of uploading files and folders to various file-sharing sites. Currently, it supports uploading to GoFile, with plans to expand to more services in the future.

## Features

- Upload files and folders to multiple file-sharing sites
- Command-line interface for easy integration into scripts and workflows
- Progress bar with real-time upload speed display
- Verbose logging option for detailed output
- Cross-platform support (Windows and Linux)

## Installation

### Pre-built Binaries

For users who prefer not to build from source, we provide pre-built binaries for both Linux and Windows. You can download the latest release from our [GitHub Releases page](https://github.com/parnexcodes/nimbus/releases).

To install:

1. Go to the [Releases page](https://github.com/parnexcodes/nimbus/releases) and find the latest release.
2. Download the appropriate binary for your system:
   - For Linux: `nimbus-linux`
   - For Windows: `nimbus-windows.exe`
3. (Linux only) Make the file executable:
   ```
   chmod +x nimbus-linux
   ```
4. Move the binary to a directory in your PATH (optional, for easier access).

### Building from Source

1. Clone the repository:
   ```
   git clone https://github.com/parnexcodes/nimbus.git
   cd nimbus
   ```

2. Build the project:
   ```
   ./build.sh
   ```

   This script will build Nimbus for both Linux and Windows. The executables will be placed in the `builds` directory.

## Usage

```
nimbus --path <FILE_OR_FOLDER_PATH> --sites <SITE1,SITE2,...> [--verbose]
```


Options:
- `--path, -p`: Path to the file or folder to upload
- `--sites, -s`: List of file sharing sites to upload to (comma-separated)
- `--verbose, -v`: Enable verbose output

Example:

```
./nimbus --path /path/to/your/file --sites gofile --verbose
```


## Supported Sites

Currently, Nimbus supports the following file-sharing sites:

- GoFile

More sites will be added in future updates.

## Development

To contribute to Nimbus or extend its functionality:

1. Fork the repository
2. Create a new branch for your feature
3. Implement your changes
4. Write tests for your new functionality
5. Submit a pull request

## License

[MIT License](LICENSE)

## Contact

For bug reports and feature requests, please open an issue on the [GitHub repository](https://github.com/parnexcodes/nimbus).
