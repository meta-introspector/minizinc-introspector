# Noob's Guide to ZOS Bootstrap FFI Build

Welcome, fellow noob! This guide will walk you through how to build the MiniZinc FFI (Foreign Function Interface) wrapper using the `zos-bootstrap` tool. This is a crucial step if you want to use MiniZinc functionalities from Rust.

## What is FFI?

FFI allows code written in one programming language (like Rust) to call functions written in another programming language (like C++). In our case, it lets Rust code talk to the MiniZinc C++ library.

## Why `zos-bootstrap`?

`zos-bootstrap` is our custom tool designed to simplify the complex build process of MiniZinc and its related components. It automates many steps that would otherwise be manual and error-prone.

## Building the FFI Wrapper

To build the FFI wrapper, you'll use the `build` command of `zos-bootstrap` with the `ffi` subcommand.

### Step 1: Navigate to the Project Root

Make sure you are in the root directory of the `libminizinc` project. You can usually tell by looking for the `Cargo.toml` file.

```bash
pwd
# Expected output: /data/data/com.termux/files/home/storage/github/libminizinc
```

### Step 2: Run the Build Command

Execute the following command in your terminal:

```bash
cargo run --package zos-bootstrap build ffi
```

Let's break down this command:
*   `cargo run`: This tells Rust's package manager (`cargo`) to run an executable.
*   `--package zos-bootstrap`: This specifies that we want to run the executable from the `zos-bootstrap` crate.
*   `build`: This is the main command we're giving to `zos-bootstrap`, indicating we want to perform a build operation.
*   `ffi`: This is a subcommand to `build`, specifically telling `zos-bootstrap` to build the FFI wrapper.

### Step 3: Verify the Build

If the command runs successfully, you should see output similar to this:

```
--- Starting build_ffi_wrapper ---
Building C++ FFI wrapper...
Project Root: /data/data/com.termux/files/home/storage/github/libminizinc
Build Dir: /data/data/com.termux/files/home/storage/github/libminizinc/build
Ensuring build_dir exists...
mkdir stdout: 
mkdir stderr: 
mkdir command executed.
Pre-condition met: build_dir exists.
Running cmake...
cmake command: cmake /data/data/com.termux/files/home/storage/github/libminizinc -DBUILD_C_WRAPPER=ON in dir /data/data/com.termux/files/home/storage/github/libminizinc/build
cmake stdout: ... (lots of cmake output) ...
cmake stderr: ... (some cmake warnings, usually okay) ...
cmake command executed.
Running make command...
make command: make minizinc_c_wrapper in dir /data/data/com.termux/files/home/storage/github/libminizinc/build
make stdout: ... (lots of make output, including [100%] Built target minizinc_c_wrapper) ...
make stderr: 
make command executed.
Checking for library at: /data/data/com.termux/files/home/storage/github/libminizinc/build/libminizinc_c_wrapper.so
Post-condition met: libminizinc_c_wrapper.so found.
C++ FFI wrapper built successfully.
--- Finished build_ffi_wrapper ---
```

The most important line is `Post-condition met: libminizinc_c_wrapper.so found.`. This confirms that the `libminizinc_c_wrapper.so` file has been successfully built and placed in the `build/` directory.

### Troubleshooting: `strace` Option

If you encounter issues and want to see what system calls are being made during the `make` process, you can use the `--strace` flag:

```bash
cargo run --package zos-bootstrap build --strace ffi
```

This will generate a `make_strace.log` file in your `build/` directory, which contains a detailed log of system calls. This log can be very helpful for advanced debugging.

## What's Next?

Now that the FFI wrapper is built, you can proceed with developing Rust code that interacts with the MiniZinc library! Happy coding!
