# pngpong

Intermediary Rust project made using guidelines from https://jrdngr.github.io/pngme_book/

This little CLI tool allows you to encode and decode messages into PNG files. 

Assuming you have rust and cargo installed on your machine (otherwise head to https://doc.rust-lang.org/cargo/getting-started/installation.html), you can go to the root of this project and run `cargo build` to install dependencies. 

The tool has 4 available commands (that need to be prefixed with `cargo run`): 

- `encode FILEPATH CHUNK_TYPE MESSAGE [OUTPUT_FILEPATH]`\
Appends a chunk of the specified type to the specified PNG file, with the message encoded in it.\
Output filepath is optional, if omitted the message will be encoded in the original file, otherwise the file will be duplicated with the name provided in the output path.\
example: `encode ~/Desktop/cat.png RuSt "I am encoding this secret message" ~/Desktop/secret_cat.png`
- `decode FILEPATH CHUNK_TYPE`\
retrieves the message (if there is one) in the first chunk with specified type\
example: `decode ~/Desktop/cat.png RuSt`

- `remove FILEPATH CHUNK_TYPE`\
removes first chunk with specified type from the file\
example: `remove ~/mountain.png TeSt`
- `print FILEPATH`\
prints the content of the PNG file in bytes\
example: `print ./test.png`

## Note about chunk types

Chunk types need to be 4-characters in length and only UTF-8 encoded letters. 
For better compatibility with PNG decoders and editors (i.e. so that your PNG file can still be read and displayed without errors), the character casing of the chunk type should be `lowercase - lowercase - uppercase - lowercase` (example: `teSt`)
If you're interested in the implementation of PNG files and their chunk types : http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html
