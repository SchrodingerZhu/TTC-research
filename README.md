# ttc
TTC calculation
## Help
```
Usage: ttc <COMMAND>

Commands:
  unshared  Caculate AET for unshared data model
  shared    Caculate AET for shared data model
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help information
```

For subcommand:
```
Caculate AET for shared data model

Usage: ttc shared [OPTIONS] --input <INPUT>

Options:
  -p, --plot <PLOT>                        Render the TTC curve
  -W, --plot-width <PLOT_WIDTH>            Plot width
  -H, --plot-height <PLOT_HEIGHT>          Plot height
  -m, --max-cache-size <MAX_CACHE_SIZE>    Maximum cache size [default: 64]
  -c, --cache-size-step <CACHE_SIZE_STEP>  Increment step of cache size [default: 1]
  -i, --input <INPUT>                      Path to the input file
  -o, --output <OUTPUT>                    Output path
  -b, --bitmap                             Use bitmap instead of SVG
  -h, --help                               Print help information
```

## Usage

```bash
cargo build --release
cd target/release
TTC_LOG=INFO ttc --input test.in -p test.svg -o test.json
```