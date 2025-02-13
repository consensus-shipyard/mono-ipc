# Deploying Blockscout on a local subnet 

Before delving into this tutorial, you should have [deployed a local subnet](deploy-a-subnet). If you're connecting to an existing remote subnet and not following the guide, make sure that you have a local docker installation.

These are instructions for deploying a basic, non-customised, local subnet explorer. The resulting instance will not provide all Blockscout features not be appropriate for production use.

1. Get Blockscout
```
git clone https://github.com/blockscout/blockscout
cd ./blockscout/docker-compose
```

2. Edit `./envs/common-blockscout.env` and set 
```
INDEXER_DISABLE_PENDING_TRANSACTIONS_FETCHER=true           
INDEXER_DISABLE_INTERNAL_TRANSACTIONS_FETCHER=true
```

The default setup assumes a local subnet with the Ethereum RPC on `localhost:8545`. If you're connecting to a remote RPC, remember to also set `ETHEREUM_JSONRPC_HTTP_URL` accordingly.

Some frontend calls use hardcoded absolute URLs. Unless you're only accessing the Blockscout interface on `localhost`, make sure to review the environmental variables in the different files under `./envs/` and adjust addresses (e.g. `BLOCKSCOUT_HOST` in `envs/common-blockscout.env` and `NEXT_PUBLIC_STATS_API_HOST` in `envs/common-frontend.env`. You'll also need to make sure the required ports are accessible (at least 80, 8080, and 8081).

3. Start Blockscout
```
docker compose -f docker-compose-no-build-geth.yml up -d
```

The web interface will be available at `http://localhost/`

If you need to take down the setup, run `docker compose -f docker-compose-no-build-geth.yml down`. Note that the data store is mounted externally and will be reused on redeployment. If you want to clear the database, run `rm -rf ./services/blockscout-db-data`.
