# dummy_gen
Generates a dummy file, of a give size in bytes. 

## Why?
In order to test _speed_ or _relaiability_ of software that interacts with the _file system_ you want to handle files of different sizez 
that contain randomly generated information. This files are commonly known as _dummy files_. Dummy in that their content is not _UTF-8_ valid, or _json_ or _XML_,
which makes them less predictable and more usefull for general testing.

## Usage

Dummy gen can be used both as a _stand alone_ app from the command line:

``
dummy_gen out.dat 1234
``
Writes 1234 _bytes_ to a file named out.dat

But then, it can also be used as a _library_, having a simple API:

```rust
use dummy_gen{write, Result};

fn main() -> Reuslt<()> {
   write(output_file_name, output_file_size)?;
}
```
