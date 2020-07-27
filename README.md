### CSV Parsing Server

This server takes a `POST` request with CSV as it's body. It parses the CSV and returns a 
JSON response containing an array with each CSV record as an object within it.

### Getting Started
Firstly, ensure you have the following installed
```
- Rust (https://www.rust-lang.org/tools/install)
```

Clone this repository and run `cargo run`. If you'd like to set any specific environment variables,
you can run `cargo run -- --help` to see all ovailable options. You can either set the environment
variable from the help text or set it in the command line, e.g. `cargo run -- -l debug -p 5000`.

The current config variables are as follows:

| env variable | CLI flag            | type   | default   | optional |
|--------------|---------------------|--------|-----------|----------|
| PORT         | -p \|\| --port      | u16    | 3000      | yes      |
| HOST         | -h \|\| --host      | String | 127.0.0.1 | yes      |
| LOG_LEVEL    | -l \|\| --log-level | String | info      | yes      |

Currently there is only one route, which is `/api/parse`. It takes a `POST` request with a CSV body.
The header `Content-Type` must be set to `text/csv` as well. The response is JSON, containing an array
of each record.

### Running locally

#### Generating some CSV
There is a basic Node script to generate a CSV.

Run this from the root directory:

```bash
cd scripts/csv_generator && npm i && npm run generate && cd -
```

For more info, please check [here](scripts/csv_generator/README.md)

#### Running the server
run `cargo run`

#### Making a request
Assuming your server is running, in another terminal, run `cd scripts` followed by `./http_req.sh`
This will make a `cURL` request to the server with the generated CSV from the step above

#### Architechture
This app is split into several crates

##### Bin
The `bin` crate is found at `src/bin`. It is the glue that calls all the other crates together to make the app :)

##### Config
The `config` crate is found at `src/config`. It contains any app level configs needed to ensure this is a 12 factor app
Any and all configs should happen here so they are centrally located and not duplicated across code.

##### Server
The `server` the main crate, located at `src/server`. It contains the code to setup and start the server as well as all the controllers. Right now,
there is only a few views (AoS for CSV records, and CSV parsing errors). It's mostly very straight forward.

One notable decision was to use an amortized vec crate. This was to ensure that as this service becomes more popular,
and we start to parse larger and larger CSV files, it is important that we can have predictable performance. By using 
an amortized vec, we can spread the cost of increasing the vector memory allocation (as it fills up) across all pushes
to the vector. While this may be a smidge slower for smaller CSV files (on the order or nanoseconds), for larger CSV files,
this will prevent re-allocation pauses.

### Current Issues
The biggest one is that currently, there is a panic due to a stack overflow somewhere in the `csv` or `serde` crate
when parsing invalid CSV. Seeing as this is a coding challenge, and the issue lies outside of this code, I am going to let it go.


The others, in no particular order:

- Configurable max body size for CSV
- Currently only takes a structured CSV, dynamic CSV is not possible at the moment, but should be trivial to implement later
    - Likely would use a HashMap to store each record
- Should stream bytes from request body into CSV buf reader as bytes are ready
    - I think using the `csv-core` crate for incremental parsing would, potentially, make this endpoint a little bit faster
- Models should be separated out into a `domain` crate for testing/readability
- Testing is not particularly robust at the moment
