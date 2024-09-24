import { createMint } from "@solana/spl-token";
import {
  Cluster,
  Commitment,
  Connection,
  Keypair,
  clusterApiUrl,
} from "@solana/web3.js";
import wallet from "../wallets/my-wba-wallet.json";

const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

const cluster: Cluster = "devnet";
const commitment: Commitment = "confirmed";
const connection = new Connection(clusterApiUrl(cluster), commitment);

(async () => {
  try {
    const mintPubkey = await createMint(
      connection,
      keypair,
      keypair.publicKey,
      null,
      6
    );
    console.log(`Created mint. Pubkey: ${mintPubkey}`);
  } catch (error) {
    console.error(`Oops, something went wrong: ${error}`);
  }
})();

/*
> Output:
Created mint. Pubkey: 5URbX6zTaujCAKLkjWAYyaHEoyjn9ntZTDr8ipcu7Cni
*/
