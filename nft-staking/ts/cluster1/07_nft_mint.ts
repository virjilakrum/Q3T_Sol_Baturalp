import {
  createNft,
  mplTokenMetadata,
} from "@metaplex-foundation/mpl-token-metadata";
import {
  createSignerFromKeypair,
  generateSigner,
  percentAmount,
  signerIdentity,
} from "@metaplex-foundation/umi";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { clusterApiUrl } from "@solana/web3.js";
import base58 from "bs58";
import wallet from "../wallets/my-wba-wallet.json";

const umi = createUmi(clusterApiUrl("devnet"));

const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(signer));

umi.use(mplTokenMetadata());

const mintSigner = generateSigner(umi);

(async () => {
  const metadataUri =
    "https://arweave.net/0vJOmN9_9fDFKFFaOmSblJh66otH67_Gxm8hBOdEGC4";
  const tx = createNft(umi, {
    mint: mintSigner,
    name: "Awesome Rug",
    symbol: "AWERUG",
    uri: metadataUri,
    sellerFeeBasisPoints: percentAmount(20.24),
  });

  const result = await tx.sendAndConfirm(umi);
  const sig = base58.encode(result.signature);
  console.log(
    `NFT created with mint account address ${mintSigner.publicKey}. Transaction signature: ${sig}`
  );
})();

/*
> Output:
NFT created with mint account address 2Fxr5F9UEkQNi3KqWgq5vQ6JMFLgrG8cNeXNngFkMeH6. Transaction signature: EnXhLQHAG76jNTDwmeCmaaBkHyYzpSCxWmpnFyJDHfFyNy2E1JpRDRrE38rtjB1ZMjagwHnSrxaG42ayjkA9ZiE
*/
