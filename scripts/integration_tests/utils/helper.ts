import * as fs from "fs";
import {
  Coin,
  isTxError,
  LCDClient,
  LegacyAminoMultisigPublicKey,
  LocalTerra,
  MnemonicKey,
  Msg,
  MsgExecuteContract,
  MsgInstantiateContract,
  MsgMigrateContract,
  MsgStoreCode,
  Wallet,
} from "@terra-money/terra.js";
import chalk from "chalk";
import { div, MATH, times } from "./math";

const network = {
  mainnet: {
    name: "mainnet",
    chainID: "phoenix-1",
    fcd: "https://fcd.terra.dev",
    lcd: "https://phoenix-lcd.terra.dev",
    nativeToken: "uluna",
    walletconnectID: 0,
  },
  testnet: {
    name: "testnet",
    chainID: "pisco-1",
    fcd: "https://pisco-fcd.terra.dev",
    lcd: "https://pisco-lcd.terra.dev",
    nativeToken: "uluna",
    walletconnectID: 0,
  },
  classic: {
    name: "mainnet",
    chainID: "columbus-5",
    fcd: "https://columbus-fcd.terra.dev",
    lcd: "https://columbus-lcd.terra.dev",
    nativeToken: "uusd",
    walletconnectID: 0,
  },
};

export const cancelSell = async (lcd: LCDClient, sender: Wallet, market: string, token_id: string, contract_address: string, class_id: string) => {
  const cancelSell = await sendTransaction(lcd, sender, [
    new MsgExecuteContract(sender.key.accAddress, market, { cancel_sell: { token_id, class_id, contract_address } }),
  ]);
  console.log({ cancelSell });
  return cancelSell;
};

export const getSellOrder = async (lcd: LCDClient, market: string, token_id: string, collection_address: string, class_id: string) => {
  const order = await lcd.wasm.contractQuery(market, { sell_order_for_token: { token_id, class_id, contract_address: collection_address } });
  console.log({ order });
  return order;
};

export const sellToken = async (
  lcd: LCDClient,
  sender: Wallet,
  market: string,
  token_id: string,
  collection_address: string,
  class_id: string,
  cw20: string
) => {
  const marketSell = {
    sell_token: {
      token_id,
      contract_address: collection_address,
      class_id,
      price: { cw20: { amount: "1000000", address: cw20 } },
    },
  };

  const marketList = await sendTransaction(lcd, sender, [
    new MsgExecuteContract(sender.key.accAddress, collection_address, {
      send_nft: {
        contract: market,
        token_id: token_id,
        msg: toEncodedBinary(marketSell),
      },
    }),
  ]);

  console.log(marketList);
  return marketList;
};

export const putOnAuction = async (
  lcd: LCDClient,
  sender: Wallet,
  auction: string,
  token_id: string,
  collection_address: string,
  class_id: string,
  cw20: string
) => {
  const auctionSell = {
    create_auction: {
      contract_address: collection_address,
      duration: 3600000,
      token_id,
      class_id,
      reserved_price: { cw20: { address: cw20, amount: "500000" } },
    },
  };

  const marketList = await sendTransaction(lcd, sender, [
    new MsgExecuteContract(sender.key.accAddress, collection_address, {
      send_nft: {
        contract: auction,
        token_id,
        msg: toEncodedBinary(auctionSell),
      },
    }),
  ]);

  console.log(marketList);
  return marketList;
};

export const whitelistContract = async (lcd: LCDClient, sender: Wallet, market: string, auction: string, collectionArray: string[]) => {
  const updateWL = await sendTransaction(lcd, sender, [
    new MsgExecuteContract(sender.key.accAddress, market, { update_whitelist: { addresses_to_add: collectionArray } }),
    new MsgExecuteContract(sender.key.accAddress, auction, { update_whitelist: { addresses_to_add: collectionArray } }),
  ]);
  return updateWL;
};

export const buyToken = async (
  lcd: LCDClient,
  sender: Wallet,
  desti_contract: string,
  amount: string,
  context_contract: string,
  token_id: string,
  class_id: string,
  collection_contract: string
) => {
  const buy = await sendTransaction(lcd, sender, [
    new MsgExecuteContract(sender.key.accAddress, context_contract, {
      send: {
        contract: desti_contract,
        amount,
        msg: toEncodedBinary({
          buy_token: {
            class_id,
            token_id,
            contract_address: collection_contract,
            // sender: ttz8.key.accAddress,
          },
        }),
      },
    }),
  ]);
  console.log(buy);
  return buy;
};

export const placeBid = async (lcd: LCDClient, sender: Wallet, auction: string, amount: string, context_contract: string, auction_id: number) => {
  const bid = await sendTransaction(lcd, sender, [
    new MsgExecuteContract(sender.key.accAddress, context_contract, {
      send: {
        contract: auction,
        amount,
        msg: toEncodedBinary({
          place_bid: {
            auction_id,
          },
        }),
      },
    }),
  ]);
  console.log(bid);
  return bid;
};

export async function setup(lcd, wallet, contractName: string, instantiateMsg: any, label: string) {
  const codeId = await storeCode(lcd, wallet, `./contracts/${contractName}.wasm`);
  console.log("codeId : ", codeId);
  await new Promise((resolve) => setTimeout(resolve, 3000));
  const contract = await instantiateContract(lcd, wallet, wallet, codeId, instantiateMsg, label);
  // console.log(JSON.stringify(contract));
  const contractAddr = contract.logs[0].events[0].attributes[0].value;
  console.log({ codeId, contractAddr });
  return { codeID: codeId, address: contractAddr };
}

export const getLcdClient = (name: string) => {
  return new LCDClient({
    chainID: network[name].chainID,
    URL: network[name].lcd,
    // gasPrices: `2${network[name].nativeToken}`,
    isClassic: name == "columbus",
  });
};

export const getLCDs = () => {
  const classic = new LCDClient({
    URL: network["classic"]?.lcd,
    chainID: network["classic"]?.chainID,
    isClassic: true,
  });

  const mainnet = new LCDClient({
    URL: network["mainnet"]?.lcd,
    chainID: network["mainnet"]?.chainID,
    isClassic: false,
  });

  const testnet = new LCDClient({
    URL: network["testnet"]?.lcd,
    chainID: network["testnet"]?.chainID,
    isClassic: false,
  });

  return { classic, mainnet, testnet };
};

export const getWallet = (lcd: LCDClient, mnemo: string | undefined = process.env.mnemonic) => {
  return lcd.wallet(new MnemonicKey({ mnemonic: mnemo }));
};

/**
 * @notice Encode a JSON object to base64 binary
 */
export function toEncodedBinary(obj: any) {
  return Buffer.from(JSON.stringify(obj)).toString("base64");
}

/**
 * @notice Send a transaction. Return result if successful, throw error if failed.
 */
export async function sendTransaction(terra: LocalTerra | LCDClient, sender: Wallet, msgs: Msg[], verbose = false) {
  await new Promise((resolve) => setTimeout(resolve, 3000));
  const tx = await sender.createAndSignTx({
    msgs,
  });

  const result = await terra.tx.broadcast(tx);

  // Print the log info
  if (verbose) {
    console.log(chalk.magenta("\nTxHash:"), result.txhash);
    try {
      console.log(chalk.magenta("Raw log:"), JSON.stringify(JSON.parse(result.raw_log), null, 2));
    } catch {
      console.log(chalk.magenta("Failed to parse log! Raw log:"), result.raw_log);
    }
  }

  if (isTxError(result)) {
    throw new Error(
      chalk.red("Transaction failed!") +
        `\n${chalk.yellow("code")}: ${result.code}` +
        `\n${chalk.yellow("codespace")}: ${result.codespace}` +
        `\n${chalk.yellow("raw_log")}: ${result.raw_log}`
    );
  }

  return result;
}

/**
 * @notice Upload contract code to LocalTerra. Return code ID.
 */
export async function storeCode(terra: LocalTerra | LCDClient, deployer: Wallet, filepath: string) {
  await new Promise((resolve) => setTimeout(resolve, 3000));
  const code = fs.readFileSync(filepath).toString("base64");
  const result = await sendTransaction(terra, deployer, [new MsgStoreCode(deployer.key.accAddress, code)]);
  return parseInt(result.logs[0].eventsByType.store_code.code_id[0]);
}

/**
 * @notice Instantiate a contract from an existing code ID. Return contract address.
 */
export async function instantiateContract(
  terra: LocalTerra | LCDClient,
  deployer: Wallet,
  admin: Wallet, // leave this emtpy then contract is not migratable
  codeId: number,
  instantiateMsg: object,
  label: string = ""
) {
  const result = await sendTransaction(
    terra,
    deployer,
    [new MsgInstantiateContract(deployer.key.accAddress, admin.key.accAddress, codeId, instantiateMsg, [], label)],
    false
  );
  return result;
}

/**
 * @notice Reset CodeId of a contract address
 *  as per doc actual method to migrate contract
 */
export async function migrateContract(
  terra: LocalTerra | LCDClient,
  deployer: Wallet,
  old_contract_addr: string,
  new_codeId: number,
  migrate_msg: any
) {
  const result = await sendTransaction(terra, deployer, [
    new MsgMigrateContract(deployer.key.accAddress, old_contract_addr, new_codeId, migrate_msg),
  ]);
  return result;
}

/**
 * Decodes a base64 string to object
 */
export function decodeBase64(base64: string) {
  return JSON.parse(Buffer.from(base64, "base64").toString());
}

/**
 * Encodes an object to base64 string
 */
export function encodeBase64(obj: object) {
  return Buffer.from(JSON.stringify(obj)).toString("base64");
}

/**
 * Encodes a u8 array into base64 string
 */
export function bytesToBase64(bytes: Uint8Array) {
  return Buffer.from(bytes).toString("base64");
}

/**
 * Decodes a base64 string to u8 array
 */
export function base64ToBytes(base64: string) {
  return new Uint8Array(Buffer.from(base64, "base64"));
}

export const microToMacro = (amount: string) => {
  return div(parseInt(amount), MATH.UNIT);
};

export const macroToMicro = (amount: string) => {
  return times(amount, MATH.UNIT);
};

export const estimateFee = async (lcd: LCDClient, address: string, msgs) => {
  try {
    const accountInfo = await lcd.auth.accountInfo(address);

    const fee = await lcd.tx.estimateFee(
      [
        {
          sequenceNumber: accountInfo.getSequenceNumber(),
          publicKey: accountInfo.getPublicKey(),
        },
      ],
      {
        msgs: msgs as unknown as Msg[],
        feeDenoms: ["uluna"],
        gasAdjustment: 1.5,
        gasPrices: [new Coin(`uluna`, `0.15`)],
        //@ts-ignore
        isClassic: lcd.config.isClassic,
      }
    );

    if (fee) {
      return fee;
    }
    throw new Error("Couldn't estimate fee");
  } catch (error) {
    console.log("error: ", error);
    throw new Error("Couldn't estimate fee");
  }
};
