# Swap ICS20

This is an *IBC Enabled* contract implements the standard ICS20 (IBC transfers), and can send custom
actions to osmosis chain, e.g. swap, join pool, exit pool.

## Messages
- `Transfer{}`: IBC Transfer
- `Swap{}`: Swap assets in Osmosis
- `JoinPool{}`: Add liquidity to a pool in Osmosis
- `ExitPool{}`: Remove liquidity to a pool in Osmosis
- `AllowExternalToken{}`: Allow external native tokens (from osmosis)

