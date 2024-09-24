import {
  createSignerFromKeypair,
  signerIdentity,
} from "@metaplex-foundation/umi";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys";
import { clusterApiUrl } from "@solana/web3.js";
import wallet from "../wallets/my-wba-wallet.json";

const umi = createUmi(clusterApiUrl("devnet"));

const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(signer));

umi.use(irysUploader());

(async () => {
  try {
    // Follow this JSON structure:
    // https://developers.metaplex.com/token-metadata#a-json-standard

    const imageUri =
      "https://arweave.net/X54lK7naxP4h_TaR1rIdxWshSjDMheU6_515vI1UpIE";
    const metadata = {
      name: "Awesome Rug",
      symbol: "AWERUG",
      description: "This rug is awesome.",
      image: imageUri,
      attributes: [
        { trait_type: "Awesomeness", value: "100" },
        { trait_type: "Rugness", value: "100" },
      ],
      properties: {
        files: [
          {
            type: "image/png",
            uri: imageUri,
          },
        ],
      },
    };

    const uri = await umi.uploader.uploadJson(metadata);
    console.log(`Metadata JSON uploaded. URI: ${uri}`);
  } catch (error) {
    console.error(`Oops, something went wrong: ${error}`);
  }
})();

/*
> Output:
Metadata JSON uploaded. URI: https://arweave.net/0vJOmN9_9fDFKFFaOmSblJh66otH67_Gxm8hBOdEGC4
*/
