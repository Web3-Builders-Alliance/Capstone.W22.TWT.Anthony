/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.24.0.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

export type ExecuteMsg = {
  deposit: {};
} | {
  redeem: {};
};
export type Uint128 = string;
export interface InstantiateMsg {
  expiration: number;
  goal: Coin;
  name: string;
  recipient: string;
}
export interface Coin {
  amount: Uint128;
  denom: string;
  [k: string]: unknown;
}
export type QueryMsg = "get_collected" | {
  get_config: {};
};