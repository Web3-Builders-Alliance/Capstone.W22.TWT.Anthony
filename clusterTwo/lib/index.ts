import type { Env } from "@terra-money/terrain";
import { ClusterTwoClient } from './clients/ClusterTwoClient';

export class Lib extends ClusterTwoClient {
  env: Env;

  constructor(env: Env) {
    super(env.client, env.defaultWallet, env.refs['clusterTwo'].contractAddresses.default);
    this.env = env;
  }
};

export default Lib;
