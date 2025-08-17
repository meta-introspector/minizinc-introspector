**FFI Build Breakthrough!** 🎉

Hey everyone, wanted to share a win from the `libminizinc` project! We've finally cracked a stubborn FFI build issue with our `zos-bootstrap` tool. Turns out, `cmake` was being a bit sneaky and writing its build files to the wrong directory, leading to elusive `.so` file problems and runtime linking errors.

We instrumented our build process with detailed logging and assertions, which quickly pinpointed the `cmake -B` argument as the key to directing the build correctly. We also took the opportunity to refactor our `build_ffi_wrapper` function into smaller, more manageable pieces, making the whole process much more robust.

This was a great lesson in systematic debugging and the power of modular design. While we still have some challenges with Rust FFI test linking on Android, the core C++ component now builds reliably!

Check out the full details in our upcoming blog post! #Rust #FFI #Debugging #MiniZinc #SoftwareEngineering #OpenSource
