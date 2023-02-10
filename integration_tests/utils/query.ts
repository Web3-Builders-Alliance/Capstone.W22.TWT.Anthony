import { LCDClient } from "@terra-money/terra.js";

export const getConfig = async (lcd: LCDClient, contract: string) => {
  await new Promise((resolve) => setTimeout(resolve, 3000));
  const config: any = await lcd.wasm.contractQuery(contract, {
    get_config: {},
  });
  console.log({ config });
  return config;
};
export const getCampaigns = async (lcd: LCDClient, contract: string) => {
  const campaigns = await lcd.wasm.contractQuery(contract, {
    get_campaigns: {},
  });
  console.log({ campaigns });
  return campaigns;
};
