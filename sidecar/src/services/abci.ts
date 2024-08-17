import {
  QueryAccountRequest,
  QueryAccountResponse,
} from "cosmjs-types/cosmos/auth/v1beta1/query.js";
import { ApiService } from "./service.js";
import { IAccountService } from "./account.js";
import { PubKey } from "cosmjs-types/cosmos/crypto/secp256k1/keys.js";
import { BaseAccount } from "cosmjs-types/cosmos/auth/v1beta1/auth.js";
import Long from "long";
import { ApiPromise } from "@pinot/api";
import { ABCIQueryResponse } from "cosmjs-types/cosmos/base/tendermint/v1beta1/query.js";
import { SimulateRequest, SimulateResponse } from "cosmjs-types/cosmos/tx/v1beta1/service.js";
import { TxService } from "./tx.js";
import { Tx } from "cosmjs-types/cosmos/tx/v1beta1/tx.js";

export class AbciService implements ApiService {
  chainApi: ApiPromise;
  accountService: IAccountService;
  txService: TxService;

  constructor(chainApi: ApiPromise, accountService: IAccountService, txService: TxService) {
    this.chainApi = chainApi;
    this.accountService = accountService;
    this.txService = txService;
  }

  async query(path: string, data: string): Promise<ABCIQueryResponse> {
    if (path === "/cosmos.auth.v1beta1.Query/Account") {
      const address = QueryAccountRequest.decode(
        Buffer.from(data, "hex")
      ).address;
      const { account } = await this.accountService.accounts(address);
      const pubkey: PubKey = {
        key: Buffer.from(account.pub_key.key, "base64"),
      };
      const baseAccount: BaseAccount = {
        address: account.address,
        pubKey: {
          typeUrl: "/cosmos.crypto.secp256k1.PubKey",
          value: PubKey.encode(pubkey).finish(),
        },
        accountNumber: Long.fromNumber(parseInt(account.account_number)),
        sequence: Long.fromNumber(parseInt(account.sequence)),
      };

      const queryAccountResponse: QueryAccountResponse = {
        account: {
          typeUrl: "/cosmos.auth.v1beta1.BaseAccount",
          value: BaseAccount.encode(baseAccount).finish(),
        },
      };
      const value = QueryAccountResponse.encode(queryAccountResponse).finish();
      const height = (await this.chainApi.query.system.number()).toString();
      return {
        code: 0,
        log: "",
        info: "",
        index: Long.ZERO,
        key: undefined,
        value,
        proofOps: undefined,
        height: Long.fromString(height),
        codespace: "",
      };
    } else if (path === '/cosmos.tx.v1beta1.Service/Simulate') {
      // TODO: Check simulate tx fields
      const request = SimulateRequest.decode(Buffer.from(data, 'hex'));
      const response = SimulateResponse.encode(await this.txService.simulate(Buffer.from(request.txBytes).toString('hex'))).finish();
      const height = (await this.chainApi.query.system.number()).toString();

      return {
        code: 0,
        log: "",
        info: "",
        index: Long.ZERO,
        key: undefined,
        value: response,
        proofOps: undefined,
        height: Long.fromString(height),
        codespace: "",
      };
    } else {
      throw new Error("unexpected path");
    }
  }
}
