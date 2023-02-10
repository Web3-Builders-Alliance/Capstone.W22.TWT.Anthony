import codegen from "@cosmwasm/ts-codegen";

codegen({
  contracts: [
    {
      name: "campaign",
      dir: "./contracts/campaign/schema",
    },
    {
      name: "factory",
      dir: "./contracts/factory/schema",
    },
    {
      name: "receipt",
      dir: "./contracts/campaign_receipt/schema",
    },
  ],
  outPath: "./integration_tests/types/",

  // options are completely optional ;)
  options: {
    bundle: {
      bundleFile: "index.ts",
      scope: "contracts",
    },
    types: {
      enabled: true,
    },
    client: {
      enabled: true,
    },
    reactQuery: {
      enabled: false,
    },
    recoil: {
      enabled: false,
    },
    messageComposer: {
      enabled: false,
    },
  },
})
  .then(() => {
    console.log("✨ all done!");
  })
  .catch(console.log);
