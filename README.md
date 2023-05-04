# us-ggg

A tool to ease creation of gas giants in Universe Sandbox

## Features

* Can check if an image is a valid gas giant
* Can convert an image into a gas giant, either overwriting the original body you pass or create a new one
* **Can print the trans flag in your terminal (if your terminal supports ANSI)**

## Usage

**For a more in-depth explanation, run `./us-ggg --help`, `./us-ggg convert --help`, etc.**

To check if an image is a valid gas giant, simply run: `./us-ggg <IMAGE> valid`.

To convert an image into a gas giant, simply run: `./us-ggg convert <IMAGE> convert <OLD_GIANT> [NEW_GIANT] [POS]`

Where `<OLD_GIANT>` is the path to the .ubox you wish to use (the object), you can find these in either:

* `C:\Documents\Universe Sandbox\Objects`, on Windows
* `~/Universe Sandbox/Objects`, on Unix-like systems (Linux/macOS)

Where `[NEW_GIANT]` is the name `us-ggg` should give the new body. Don't pass this to overwrite the original body.

Where `[POS]` is the column of your image to use if it has a width other than 1.

After this, restart Universe Sandbox and your gas giant will be available!

## Installation

Simply extract the archive in releases corresponding to your OS.

* aarch64-apple-darwin - macOS Silicon
* x86_64-apple-darwin - macOS x86_64
* x86_64-pc-windows-gnu - Windows
* x86_64-unknown-linux-gnu - Linux

On Unix-like systems, you may need to do some extra setup; run `chmod +x us-ggg` from your terminal while in its directory. This will give it executable permission. You should also be able to do this from your file explorer.

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
