## A toy browser rendering engine

A browser rendering engine is a portion of a web browser that works under the hood to fetch a web page from the internet and translate its content into forms you read, watch and hear, etc

Examples of rendering engine used in popular browsers:

1. Firefox --> Gecko
2. Chrome --> Blink
3. WebKit --> Safari

Built with:

- Rust

### Quick start

- Install Rust
- Clone the source code
- Run `cargo build` to build the browser engine and `cargo run` to run it

By default the rendering engine will load test.html and test.css from the `examples` directory. You can use the `--html or -h` and `--css or -h` arguments to the engine executable to change the input files:

```bash
./target/debug/browser-engine --html examples/test.html --css examples/test.css

```

The output file will be saved to a file called `output.png`
