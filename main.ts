import { MsgExecuteContract } from "@terra-money/terra.js";
import { getLCDs, getWallet, sendTransaction } from "./integration_tests/utils/helper";
import { getCampaigns, getConfig } from "./integration_tests/utils/query";
import { setupContracts, instantiateFactory, createCampaign } from "./integration_tests/utils/setup";

require("dotenv").config();
import * as chai from "chai";

const proper_instantiation = async (lcd, wallet) => {
  try {
    const { factoryID, campaignID, receiptID, cw20ID } = await setupContracts(lcd, wallet);

    const factory = await instantiateFactory(lcd, wallet, factoryID, {
      admin: wallet.key.accAddress,
      code_ids: {
        campaign: campaignID,
        receipt: receiptID,
        cw3: 0,
        cw20: cw20ID,
      },
    });

    const config = await getConfig(lcd, factory);

    return { config, factory };
  } catch (error) {
    console.log(error);
  }
};

(async () => {
  try {
    const lcds = getLCDs();

    const wallet = getWallet(lcds["testnet"], process.env.MNEMONIC);
    const sender = wallet.key.accAddress;

    const { config, factory } = {
      config: {
        admin: "terra16eey0m7w6gn7ekq944nzaw5uzjxe0retuv89ta",
        code_ids: { campaign: 7543, cw3: 0, cw20: 6409, receipt: 7544 },
      },
      factory: "",
    };
    // (await proper_instantiation(lcds["testnet"], wallet)) as any;

    const details1 = await createCampaign(lcds["testnet"], wallet, factory, {
      name: "test",
      expiration: 1701471600,
      goal: {
        denom: "uluna",
        amount: "1000000000",
      },
      recipient: sender,
    });

    const details2 = await createCampaign(lcds["testnet"], wallet, factory, {
      name: "test",
      expiration: 1696197600,
      goal: {
        denom: "ujuno",
        amount: "5000000",
      },
      recipient: sender,
    });

    console.log({ details1, details2 });

    // const { factoryID, campaignID, receiptID, cw20ID } = { factoryID: 7532, campaignID: 7533, receiptID: 7534, cw20ID: 6409 };
    // await setupContracts(lcds["testnet"], wallet);

    // await instantiateFactory(lcds["testnet"], wallet, factoryID, {
    //   admin: sender,
    //   code_ids: {
    //     campaign: campaignID,
    //     receipt: receiptID,
    //     cw3: 0,
    //     cw20: cw20ID,
    //   },
    // });

    // const { campaign_addr, receipt_addr, token_addr } = {
    //   campaign_addr: "terra1et8jyt77589f96j3vqtey0wm66pct0yadkv00cwn4a3dyzhe573seztakt",
    //   receipt_addr: "terra1tphlmn6m0cn3pkjr6z7wnvay3qhm87nm8mmh9ht798tg364zj40sy0nkc2",
    //   token_addr: "terra1xxwe30449nactl52trp43ny5jn6a7kgs0z7mezag848jdsfwenaq9n5c4p",
    // };
    // await createCampaign(lcds["testnet"], wallet, factory, {
    //   name: "test",
    //   expiration: 1677452400,
    //   goal: {
    //     denom: "uluna",
    //     amount: "1000000000",
    //   },
    //   recipient: sender,
    // });

    // getCampaigns(lcds["testnet"], factory);
  } catch (e) {
    console.error(e);
  }
})();
