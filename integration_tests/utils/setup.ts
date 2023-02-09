import { LCDClient, MsgExecuteContract, Wallet } from "@terra-money/terra.js";
import { instantiateContract, sendTransaction, storeCode } from "./helper";

type initCampaignMsg = {
  name: string;
  expiration: number;
  goal: { denom: string; amount: string };
  recipient: string;
};

type initFactoryMsg = {
  admin: string;
  code_ids: {
    campaign: number;
    receipt: number;
    cw3: number;
    cw20: number;
  };
};

export const setupContracts = async (lcd, wallet) => {
  const factoryID = await storeCode(lcd, wallet, "./artifacts/factory.wasm");
  const campaignID = await storeCode(lcd, wallet, "./artifacts/campaign.wasm");
  const receiptID = await storeCode(lcd, wallet, "./artifacts/campaign_receipt.wasm");
  const cw20ID = 6409;
  console.log({ factoryID, campaignID, receiptID, cw20ID });
  return { factoryID, campaignID, receiptID, cw20ID };
};

export const instantiateFactory = async (lcd: LCDClient, sender: Wallet, factoryID: number, initMsg: initFactoryMsg) => {
  const factory = await instantiateContract(lcd, sender, sender, factoryID, initMsg, "factory");

  const addr = factory.logs[0].events[0].attributes[0].value;
  console.log({ factory: addr });
  return addr;
};

export const createCampaign = async (lcd: LCDClient, sender: Wallet, factory_contract: string, initCampaignMsg: initCampaignMsg) => {
  const createCampaign = await sendTransaction(lcd, sender, [
    new MsgExecuteContract(sender.key.accAddress, factory_contract, {
      create_campaign: initCampaignMsg,
    }),
  ]);
  console.log(createCampaign);
  return createCampaign;
};
