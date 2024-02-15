# Usage

```bash
Usage: send-usdt --private-key <PRIVATE_KEY> --to <TO> --value <VALUE> --infura-key <INFURA_KEY> --extra-data <EXTRA_DATA>

Options:
  -p, --private-key <PRIVATE_KEY>  Private key of the account from which to transfer USDT
  -t, --to <TO>                    Destination account
  -v, --value <VALUE>              USDT value to transfer
  -i, --infura-key <INFURA_KEY>    Infura access key
  -e, --extra-data <EXTRA_DATA>    Extra data to append to contract call
  -h, --help                       Print help
  -V, --version                    Print version
```

# Example

```bash
cargo run -- \
       --private-key "0xafcf028e89aff61be82c4dfa1a22ed0e8a761d507323f90861d608536455357d" \
       --to "0xE4184173A474d81Bd9df371d84B6410f47aEDD7E" \
       --value 1000000 \
       --infura-key "92dbf988a17b7336d6960f27a083ef09" \
       --extra-data "Hello world!"
```
