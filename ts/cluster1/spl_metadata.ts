import wallet from "./wallet/wba-wallet.json";

import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
  createMetadataAccountV3,
  CreateMetadataAccountV3InstructionAccounts,
  CreateMetadataAccountV3InstructionArgs,
  DataV2Args,
} from "@metaplex-foundation/mpl-token-metadata";
import {
  createSignerFromKeypair,
  signerIdentity,
  publicKey,
} from "@metaplex-foundation/umi";
import { findMetadataPda } from "@metaplex-foundation/mpl-token-metadata";
import base58 from "bs58";

// Define our Mint address
const mint = publicKey("HWkiywmVgVmVzg3JtHivLrTrQrrKjMaP7mR8QKk84F7b");

// Create a UMI connection
const umi = createUmi("https://api.devnet.solana.com");
const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(createSignerFromKeypair(umi, keypair)));

(async () => {
  try {
    const metadataPda = findMetadataPda(umi, { mint });

    const accounts: CreateMetadataAccountV3InstructionAccounts = {
      metadata: metadataPda,
      mint: mint,
      mintAuthority: signer,
      payer: signer,
      updateAuthority: signer.publicKey,
    };

    const data: DataV2Args = {
      name: "zklxrugs",
      symbol: "ZKLX",
      uri: "",
      sellerFeeBasisPoints: 500,
      creators: null,
      uses: null,
      collection: null,
    };

    const args: CreateMetadataAccountV3InstructionArgs = {
      data: data,
      isMutable: true,
      collectionDetails: null,
    };

    let tx = createMetadataAccountV3(umi, {
      ...accounts,
      ...args,
    });

    let result = await tx.sendAndConfirm(umi);

    console.log(base58.encode(result.signature));
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();
