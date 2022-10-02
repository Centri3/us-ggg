# us-ggg

A CLI tool for creating Universe Sandbox gas giants

## Usage

To convert an image into a gas giant, simply run (while in the same working directory as the executable and image): `us-ggg convert -i=image_name_here -o=output_name_here --pos=pos_here`

To check if an image is a valid gas giant, simply run: `us-ggg valid -i=image_name_here`

* You then need to open the ubox archive containing your bodies' properties, which is usually residing in `C:\Users\You\Documents\Universe Sandbox\Objects` on Windows.
* Replace the `Colors:[]` array in `object.json` with the contents of the outputted json from ggg.
* Restart `Universe Sandbox`.

If this is too complex, a GUI is also planned for ggg, but this will not be done for a while (or maybe ever (: )

## Installation

Simply extract the archive in releases corresponding to your OS.

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
