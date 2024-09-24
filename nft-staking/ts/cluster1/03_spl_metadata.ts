import {
  createMetadataAccountV3,
  CreateMetadataAccountV3InstructionAccounts,
  CreateMetadataAccountV3InstructionArgs,
  DataV2Args,
} from "@metaplex-foundation/mpl-token-metadata";
import {
  createSignerFromKeypair,
  publicKey,
  signerIdentity,
} from "@metaplex-foundation/umi";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { clusterApiUrl } from "@solana/web3.js";
import bs58 from "bs58";
import wallet from "../wallets/my-wba-wallet.json";

const mintPubkey = publicKey("5URbX6zTaujCAKLkjWAYyaHEoyjn9ntZTDr8ipcu7Cni");

const umi = createUmi(clusterApiUrl("devnet"));
const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(signer));

(async () => {
  try {
    const accounts: CreateMetadataAccountV3InstructionAccounts = {
      mint: mintPubkey,
      mintAuthority: signer,
      updateAuthority: signer.publicKey,
    };

    const data: DataV2Args = {
      name: "Osi Token",
      symbol: "OSI",
      uri: "",
      sellerFeeBasisPoints: 0,
      creators: null,
      collection: null,
      uses: null,
    };

    const args: CreateMetadataAccountV3InstructionArgs = {
      data,
      isMutable: true,
      collectionDetails: null,
    };

    const tx = createMetadataAccountV3(umi, {
      ...accounts,
      ...args,
    });

    const result = await tx.sendAndConfirm(umi);
    console.log(
      `Created metadata account. Transaction signature: ${bs58.encode(
        result.signature
      )}`
    );
  } catch (error) {
    console.error(`Oops, something went wrong: ${error}`);
  }
})();

/*
> Output:
Created metadata account. Transaction signature: 5P6PwAdpmdk1zEWquwWi9FLFJhnm3gM1vU7aRgjpEoFdaAuQB91ZMwReURSzXZMux93FRNsnm57FrduPvqCN1t1f
*/
