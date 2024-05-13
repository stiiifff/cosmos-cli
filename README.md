# Cosmos CLI

The Cosmos CLI is a simple command-line tool that provides useful commands to get information about various resources in the [Cosmos Blockchain ecosystem](https://cosmos.network/).

## Installation
First ensure that you have a fairly recent version of rust/cargo installed. Then, execute the following command in your shell:
```console,ignore
$ cargo install cosmos-cli
```

## Usage

To get information about the available commands, run:
```console,ignore
$ cosmos_cli --help

Cosmos CLI

Usage: cosmos-cli [OPTIONS] <COMMAND>

Commands:
  info       Get information about resources in the Cosmos ecosystem
  list       List resources in the Cosmos ecosystem
  astroport  Get information about Astroport resources
  help       Print this message or the help of the given subcommand(s)

Options:
  -o, --output <OUTPUT>  Output format [default: plain] [possible values: plain, json]
  -h, --help             Print help
  -V, --version          Print version
```

To get the list of all chains in the Cosmos ecosystem:
```console,ignore
$ cosmos-cli list chains

8ball
acrechain
agoric
aioz
...
```
To get the result formatted as JSON, use the `--output` parameter:
```console,ignore
$ cosmos-cli --output json list chains

["8ball","acrechain","agoric","aioz","akash","akiro", "andromeda","andromeda1", ... ,"xpla","zetachain"]
```
The JSON-formatted output is very handy when combined with [jq](https://jqlang.github.io/jq/).

The output will be formatted as a table for the `info` command.
As an example, to display information about the CosmosHub's ATOM token:
```console,ignore
$ cosmos-cli info asset cosmoshub atom

+--------------+--------------+
| base         | uatom        |
+--------------+--------------+
| coingecko_id | cosmos       |
+--------------+--------------+
| decimals     | 0            |
+--------------+--------------+
| display      | atom         |
+--------------+----------+---+
...
```

There's plenty more useful commands, learn about all of them in the next section.
And if you see one that's missing, please submit a PR ! :)

## Available subcommands

### `cosmos-cli info`

```console,ignore
Get information about resources in the Cosmos ecosystem

Usage: cosmos-cli info [OPTIONS] <COMMAND>

Commands:
  chain   Get information about a specific chain in the Cosmos ecosystem
  assets  Get information about all assets of a specific chain in the Cosmos ecosystem
  asset   Get information about a specific asset of a specific chain in the Cosmos ecosystem
  path    Get information about all IBC paths in the Cosmos ecosystem
```

#### Example

```console,ignore
$ cosmos-cli info path neutron stargaze

+----------+---------------+---------------------+
| chain_1  | chain_name    | neutron             |
|          +---------------+---------------------+
|          | client_id     | 07-tendermint-31    |
|          +---------------+---------------------+
|          | connection_id | connection-23       |
+----------+---------------+---------------------+
| chain_2  | chain_name    | stargaze            |
|          +---------------+---------------------+
|          | client_id     | 07-tendermint-283   |
|          +---------------+---------------------+
|          | connection_id | connection-211      |
+----------+----------+----+-------+-------------+
| channels | chain_1  | channel_id | channel-18  |
|          |          +------------+-------------+
|          |          | port_id    | transfer    |
|          +----------+------------+-------------+
|          | chain_2  | channel_id | channel-191 |
|          |          +------------+-------------+
|          |          | port_id    | transfer    |
|          +----------+------------+-------------+
|          | ordering | unordered                |
|          +----------+-----------+--------------+
|          | tags     | preferred | true         |
|          |          +-----------+--------------+
|          |          | status    | live         |
|          +----------+-----------+--------------+
|          | version  | ics20-1                  |
+----------+----------+--------------------------+
```

### `cosmos-cli list`

```console,ignore
List resources in the Cosmos ecosystem

Usage: cosmos-cli list [OPTIONS] <COMMAND>

Commands:
  chains  List all chains in the Cosmos ecosystem
  assets  List assets for a specific chain in the Cosmos ecosystem
  paths   List all IBC paths in the Cosmos ecosystem
```

#### Example

```console,ignore
$ cosmos-cli list assets osmosis | wc -l
344

$ cosmos-cli list assets osmosis

OSMO
ION
USDC.axl
ETH
WBTC.axl
USDT.axl
DAI
BUSD
ATOM
CRO
BNB
MATIC
AVAX
...


```

### `cosmos-cli astroport`

```console,ignore
Get information about Astroport resources

Usage: cosmos-cli astroport [OPTIONS] <COMMAND>

Commands:
  native-tokens  List native tokens on Astroport
  pairs          List pairs on Astroport
  pair           Get information about a specific pair on Astroport
  pool           Get information about a specific pool on Astroport
```

#### Example

Getting info about the STARS/NTRN pool on Astroport:

```console,ignore
$ cosmos-cli astroport pool ibc/A139C0E0B5E87CBA8EAEEB12B9BEE13AC7C814CFBBFA87BBCADD67E31003466C untrn
+-------------+--------+---------------------------------------------------------------------------------------------+
| assets      | amount | 4458687253                                                                                  |
|             +--------+--------------+-------+----------------------------------------------------------------------+
|             | info   | native_token | denom | ibc/A139C0E0B5E87CBA8EAEEB12B9BEE13AC7C814CFBBFA87BBCADD67E31003466C |
|             +--------+--------------+-------+----------------------------------------------------------------------+
|             | amount | 126374360                                                                                   |
|             +--------+--------------+-------+----------------------------------------------------------------------+
|             | info   | native_token | denom | untrn                                                                |
+-------------+--------+--------------+-------+----------------------------------------------------------------------+
| total_share | 749090358                                                                                            |
+-------------+------------------------------------------------------------------------------------------------------+
```

## License
The Cosmos CLI is licensed under [Apache 2](LICENSE).
