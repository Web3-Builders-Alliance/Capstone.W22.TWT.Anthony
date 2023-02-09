import { MsgExecuteContract } from "@terra-money/terra.js";
import { getLCDs, getWallet, sendTransaction } from "./integration_tests/utils/helper";

require("dotenv").config();

(async () => {
  try {
    const lcds = getLCDs();

    const wallet = getWallet(lcds["testnet"], process.env.MNEMONIC);
    const sender = wallet.key.accAddress;

    const { factoryID, campaignID, receiptID, cw20ID } = { factoryID: 7520, campaignID: 7521, receiptID: 7522, cw20ID: 6409 };
    // await setupContracts(lcds["testnet"], wallet);

    const factory = "terra158aqzw6c0dsw0fk6p8uvm0j04epd52rlgsejs7d97x2cdlca5qcslzr20f";
    // await instantiateFactory(lcds["testnet"], wallet, factoryID, {
    //   admin: sender,
    //   code_ids: {
    //     campaign: campaignID,
    //     receipt: receiptID,
    //     cw3: 0,
    //     cw20: cw20ID,
    //   },
    // });

    // instantiate campaign & receipt
    const createCampaign = await sendTransaction(lcds["testnet"], wallet, [
      new MsgExecuteContract(sender, factory, {
        create_campaign: {
          name: "test",
          expiration: 1677452400,
          goal: { denom: "uluna", amount: "1000000000" },
          recipient: sender,
        },
      }),
    ]);
    console.log(createCampaign);
  } catch (e) {
    console.error(e);
  }
})();
