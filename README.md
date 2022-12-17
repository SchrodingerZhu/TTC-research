# ttc
TTC calculation
## Help
```
Usage: ttc [OPTIONS] --input <INPUT>

Options:
-p, --plot <PLOT>                        Render the TTC curve
-W, --plot-width <PLOT_WIDTH>            Plot width
-H, --plot-height <PLOT_HEIGHT>          Plot height
-m, --max-cache-size <MAX_CACHE_SIZE>    Maximum cache size [default: 64]
-c, --cache-size-step <CACHE_SIZE_STEP>  Increment step of cache size [default: 1]
-i, --input <INPUT>                      Path to the input file
-o, --output <OUTPUT>                    Output path
-h, --help                               Print help information
```

## Usage

```bash
cargo build --release
cd target/release
TTC_LOG=INFO ttc --input test.in -p test.svg -o test.json
```