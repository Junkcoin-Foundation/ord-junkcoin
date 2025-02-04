# Ord-Junkcoin

ℹ️ This is a fork/based Dogecoin wonky-ord

## Key differences

‼️ DISCLAIMER: OUR CODE MAY STILL HAVE BUGS️

We included the real wonky block rewards from block 0 until block 144,999. We invite you to critically review our code in `src/epoch.rs`. We are convinced that Junkinals should use actual block rewards instead of a simplified version.

## API documentation
You can find the API documentation [here](openapi.yaml).
Most convenient way to view the API documentation is to use the [Swagger Editor](https://editor.swagger.io/).
You can import the `openapi.yaml` file and view the API documentation via Import URL: `https://raw.githubusercontent.com/Junkcoin-Foundation/ord-junkcoin/main/openapi.yaml`.

## TL;DR How to run

### Download Pre-build `index.redb` for Ord Server (credit Lomiamai)

Make sure to download prebuilt `index.redb` [here]([openapi.yaml](https://drive.google.com/file/d/14LfaHrnZE12CvBSB-0jxKQu_8TYp-cH0/view?usp=sharing)) before proceed to ord indexing.
Download `ord-index.zip` and unzip the file `index.redb`

```bash
# Install gdown for google largr file download
pip install gdown

# Download ord-index.zip gdown
gdown --fuzzy "https://drive.google.com/file/d/14LfaHrnZE12CvBSB-0jxKQu_8TYp-cH0/view?usp=sharing" -O ord-index.zip

# Then unzip and move to ord index path
mkdir -p /mnt/ord-node/indexer-data-main
unzip ord-index.zip -d temp_dir && \
mv temp_dir/index.redb /mnt/ord-node/indexer-data-main/ && \
rm -r temp_dir

# Verify the file is in place
ls -l /mnt/ord-node/indexer-data-main/index.redb
```

### Preqrequisites
You will have to launch your own Junkcoin node and have it fully synced. You can use the following guide to set up your own Junkcoin node:
1. Download latest version from [Junkcoin](https://github.com/Junkcoin-Foundation/junkcoin-core/releases) and install it.
   1. We have tested and launched the indexer with Junkcoin Core v1.14.8.
2. Follow the [installation instructions](https://github.com/Junkcoin-Foundation/junkcoin-core/blob/master/INSTALL.md)
   1. We started the Junkcoin Core with the following flags:
      ```shell
      junkcoind -txindex -rpcuser=foo -rpcpassword=bar -rpcport=19771 -rpcallowip=0.0.0.0/0 -rpcbind=127.0.0.1
      ```
   2. Make sure your Junkcoin node is fully synced before starting the indexer.
   3. ‼️ **IMPORTANT** Ensure to replace `foo` and `bar` with your own username and password. **IMPORTANT** ‼️
3. Start the indexer with rpc-url pointing to your Junkcoin node and the data-dir pointing to the directory where the indexer should store its data.

```shell

### Start the ord indexer / server
```shell
export RUST_LOG=info
// Set the path to the subsidies.json and starting_sats.json files
export SUBSIDIES_PATH=/home/junkuser/ord-junkcoin/subsidies.json
export STARTING_SATS_PATH=/home/junkuser/ord-junkcoin/starting_sats.json

# ensure the data directory exists
mkdir -p /mnt/ord-node/indexer-data-main

# replace YOUR_RPC_URL with the URL of your Junkcoin node like: http://foo:bar@127.0.0.1:19771

// Start Indexing
ord --rpc-url=YOUR_RPC_URL --data-dir=/mnt/ord-node/indexer-data-main --nr-parallel-requests=16 --first-inscription-height=4609723 --first-june-height=5084000 --index-junes --index-transactions --index-junk20 index

// Start Indexing + Server
ord --rpc-url=YOUR_RPC_URL --data-dir=/mnt/ord-node/indexer-data-main --nr-parallel-requests=16 --first-inscription-height=4609723 --first-june-height=5084000 --index-junes --index-transactions --index-junk20 server
```
`--index-transactions` will store transaction data, this is currently needed for `--index-junk20` and furthermore helps
for a better performance for the API.
`--nr-parallel-requests` will configure how many parallel requests while indexing are sent to your RPC Server - 16 is
recommended for default node settings.

With all settings enabled, the database will currently need around 400gb when fully indexed.

### Required env vars

On the root level of this repo you'll find a `subsidies.json` and `starting_sats.json` file. When starting ord you will need to set the location of these files to env variables.

Example:
If your `ord-junkcoin` dir is `/home/junkuser/ord-junkcoin` then set the vars:
`SUBSIDIES_PATH=/home/junkuser/ord-junkcoin/subsidies.json`
and
`STARTING_SATS_PATH=/home/junkuser/ord-junkcoin/starting_sats.json`.

## Start the ord indexer / server in Docker
You can use a docker image to run the ord indexer / server.

### Prerequisites Docker
1. Use ubuntu linux or a similar distribution
2. Install junkcoind and have it fully synced
   1See [Junkcoin installation instructions](#preqrequisites)
3. Install docker and docker-compose (Ubuntu)[https://docs.docker.com/engine/install/ubuntu/]
4. Clone this repository

### Build the Docker image
```shell
./docker_build.sh
```

### Start the ord in a docker container
```shell
docker-compose up -d
```

### Stop the ord in a docker container
When stopping the ord in a container it is important to add a timeout.
If no timeout is add, the process cannot close the database properly and the next start will take ages or fail.

```shell
docker-compose stop -t 600
docker-compose down
```

## Original README
Please check the original [README](READMEFROMAPEZORD.md) for more information on how to run `ord` and the required env vars.
