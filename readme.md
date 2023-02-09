First intent with this project is/was to decentralize fund raising

Overview:

Users should be able create fund raising campaign easily:

- Before launch they set the campaign parameters namely Expiration & Fund threshold to be reach, will the project need a cw20 etc...
- Once campaign is launched and while it's not expired, any address that send funds to the campaign will receive the associated nft as receipt, funds will be escrowed there <!-- TODO add some emergency exit -->
- On expiration, if threshold is reach funds are claimable by campaign creator <!-- ?! TODO add cw3 as anti rug pollicy ?! -->
  else funds are sent back to their original wallet <!-- TODO might be costly gas wise shall user claim their refund instead -->

alternative scenario with cw20 instantiation:
if needed creator can set campaign to create a cw20 on launch that will be fully vested
cw20 will be claimable once vesting is complete

<!-- using merkleroot approach might be a good idea there -->

Contracts:

Campaign:

- is the only entity able to update factory params

Factory:

- is the only entity able to instantiate:
  -- campaign contract
  -- associated cw721 and/or cw20 contracts

Campaign_receipt:

- is a mutable non transferable cw721
- is emitted on every first deposit, subsequent deposits only update token metadata

Vesting contract => cw_payroll?

- cw20 token: perks for investors will be vested over time
- campaign balance: will also be vested

admin should be able to freeze/claw back if necessary
