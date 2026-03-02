# sta-vi

Small SAT solver in [Vine](https://vine.dev/). Currently it implement the most basic DPLL algorithm.

## Run

You can run the examples in the test folder with e.g.

```sh
vine run sat/main.vi --lib sat < tests/medium/xor_parity_mix_sat.cnf
```

## Test

```sh
nix flake check
```
