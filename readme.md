# Native File Manager

A native desktop file manager built with **Rust** and **Slint**, designed to provide a clean, responsive, and efficient way to browse and manage files and directories.

The application aims to deliver a Windows File Explorer-style experience while leveraging Rust's performance, safety, and cross-platform development capabilities.

## Features

### File and Directory Navigation
- Browse files and directories on the local filesystem.
- Navigate into directories by opening them.
- Navigate to parent directories using the Up button.
- Navigate backward through previously visited locations.
- Browse directory paths using breadcrumbs.
- Open directories by entering their paths manually.

### File Information
- Display file and directory names.
- Display file sizes in a human-readable format.
- Show file extensions.
- Display last-modified dates.
- Use file-type icons to make entries easier to identify.
- Sort directories before files and organize entries alphabetically.

### File Selection and Context Menu
- Select individual files and directories.
- Display a context menu with actions such as:
  - Open
  - Rename
  - Copy path
  - Properties
  - Delete

*Note: Some context-menu actions may still be under development.*

### Drive Overview
The planned starting screen provides an overview of available drives, similar to Windows File Explorer's **This PC** section.

The intended drive overview includes:
- Available drives and mounted volumes.
- Drive names and mount points.
- Filesystem information.
- Total storage capacity.
- Used storage space.
- Available storage space.
- Storage usage indicators.

## Technology Stack

| Technology | Purpose |
|---|---|
| Rust | Core application logic and filesystem operations |
| Slint | Native desktop user interface |
| Cargo | Dependency and project management |
| Chrono | File modification date formatting |
| Arboard | Clipboard integration |
| Sysinfo | System and disk information (planned/integration dependent) |

## Project Structure

```text
file-manager/
├── Cargo.toml
├── Cargo.lock
├── build.rs
└── src/
    ├── main.rs
    └── ui/
        ├── main.slint
        └── file.slint
```

### Structure Overview

- `main.rs` — Application entry point, filesystem operations, navigation logic, and communication between Rust and Slint.
- `ui/main.slint` — Main application window and user interface components.
- `ui/file.slint` — Shared data structures for files, breadcrumbs, and other filesystem-related UI models.
- `build.rs` — Compiles the Slint interface during the Cargo build process.

## Requirements

- Rust toolchain
- Cargo
- Windows, Linux, or macOS development environment
- Slint-compatible build dependencies for your target platform

## Installation and Setup

### 1. Clone the repository

```bash
git clone <repository-url>
cd file-manager
```

### 2. Build the application

```bash
cargo build
```

### 3. Run the application

```bash
cargo run
```

For an optimized release build:

```bash
cargo build --release
```

The compiled executable will be available in:

```text
target/release/
```

## Development

The application separates its user interface from its core filesystem logic.

Rust handles:
- Filesystem access and directory traversal.
- File metadata retrieval.
- Navigation state and history.
- File and directory operations.
- Clipboard integration.
- System and storage information.

Slint handles:
- Window layout and styling.
- File and directory listings.
- Navigation controls.
- Breadcrumb navigation.
- File selection and context-menu interactions.

This separation makes it easier to extend the application without tightly coupling the interface to filesystem operations.

## Roadmap

Planned improvements include:

- [ ] Drive overview with storage usage indicators.
- [ ] File and directory creation.
- [ ] Rename, copy, move, and delete operations.
- [ ] File properties dialog.
- [ ] File search.
- [ ] File filtering and advanced sorting.
- [ ] Multiple tabs.
- [ ] Split-pane file browsing.
- [ ] Drag-and-drop support.
- [ ] Keyboard shortcuts.
- [ ] File preview.
- [ ] Background operations for large file transfers.
- [ ] Improved error handling and operation progress indicators.
- [ ] Cross-platform testing and packaging.

## Project Goals

The primary goal is to build a capable native desktop file manager with a clean interface and reliable filesystem