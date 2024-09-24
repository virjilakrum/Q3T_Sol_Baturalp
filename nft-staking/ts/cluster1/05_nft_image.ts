import {
  createGenericFile,
  createSignerFromKeypair,
  signerIdentity,
} from "@metaplex-foundation/umi";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys";
import { clusterApiUrl } from "@solana/web3.js";
import { readFile } from "fs/promises";
import wallet from "../wallets/my-wba-wallet.json";

const umi = createUmi(clusterApiUrl("devnet"));

const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(signer));

umi.use(irysUploader());

(async () => {
  try {
    const image = await readFile("./assets/generug.png");

    const imageGenericFile = createGenericFile(image, "generug.png", {
      contentType: "image/png",
    });

    const [uri] = await umi.uploader.upload([imageGenericFile]);
    console.log(`Image uploaded. URI: ${uri}`);
  } catch (error) {
    console.error(`Oops, something went wrong: ${error}`);
  }
})();

/*
> Output:
Image uploaded. URI: https://arweave.net/X54lK7naxP4h_TaR1rIdxWshSjDMheU6_515vI1UpIE
*/
