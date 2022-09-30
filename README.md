# Vaidyuti Marketprice Middleware

The marketprice middleware project uses `rust`.

## How to use?

Create a `.env` file and specify the `HOST` and `PORT` variables.

```env
HOST=test.mosquitto.org
PORT=1883
```

Then `cargo run` the project.

## How to compile?

Requires `cargo` to compile the binary which is avaliable along with `rust`. Make sure to use the `--release` flag for faster builds.

```bash
git clone https://github.com/vaidyuti/marketprice-middleware.git
cd marketprice-middleware
cargo build --release
```
