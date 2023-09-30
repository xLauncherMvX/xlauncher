PROJECT="${PWD}"

CORE_LOGS="interaction/logs"
MY_DECIMALS="000000000000000000"
MY_BYTECODE="output/xlauncher-simple.wasm"

INITIAL_PRICE="1000${MY_DECIMALS}"

setEnvDevnet() {
  CURRENT_ENV="devnet"
  ENV_LOGS="${CORE_LOGS}/${CURRENT_ENV}"

  cp -f mxpy.data-storage-devnet.json mxpy.data-storage.json
  PEM_FILE="${PROJECT}/../../../wallets/devnet_owner_wallet.pem"
  ADDRESS=$(mxpy data load --key=address-devnet)
  PROXY=https://devnet-gateway.multiversx.com
  CHAINID=D

  TOKEN_ID="XLH-4a7cc0"
  START_TIME_STAMP=$(date -d '2023-10-01 00:00:00' +"%s")

  TOKEN_ID_HEX=$(echo -n ${TOKEN_ID} | xxd -p)
  SFT_ID_HEX=$(echo -n ${SFT_ID} | xxd -p)
}

deployContract() {
  MY_LOGS="${ENV_LOGS}-deploy.json"
  mxpy --verbose contract deploy --bytecode ${MY_BYTECODE} --recall-nonce --pem=${PEM_FILE} \
    --gas-limit=100000000 --send --outfile="${MY_LOGS}" \
    --proxy=${PROXY} --chain=${CHAINID} || return

  TRANSACTION=$(mxpy data parse --file="${MY_LOGS}" --expression="data['emitted_tx']['hash']")
  ADDRESS=$(mxpy data parse --file="${MY_LOGS}" --expression="data['emitted_tx']['address']")

  mxpy data store --key=address-devnet --value=${ADDRESS}
  mxpy data store --key=deployTransaction-devnet --value=${TRANSACTION}

  echo ""
  echo "Smart contract address: ${ADDRESS}"
}

updateContract() {
  MY_LOGS="${ENV_LOGS}-updateContract.json"
  mxpy --verbose contract upgrade ${ADDRESS} --bytecode ${MY_BYTECODE} --recall-nonce --pem=${PEM_FILE} \
    --gas-limit=100000000 --send --outfile="${MY_LOGS}" \
    --proxy=${PROXY} --chain=${CHAINID}
}

setContractSettings(){
  MY_LOGS="${ENV_LOGS}-setContractSettings.json"
  mxpy --verbose contract call ${ADDRESS} --recall-nonce \
    --pem=${PEM_FILE} \
    --gas-limit=8000000 \
    --proxy=${PROXY} --chain=${CHAINID} \
    --function="setContractSettings" \
    --arguments "0x${TOKEN_ID_HEX}" ${INITIAL_PRICE} ${START_TIME_STAMP} \
    --send \
    --outfile="${MY_LOGS}"
}

