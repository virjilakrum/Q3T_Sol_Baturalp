import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";
import { Connection, Keypair, PublicKey, clusterApiUrl } from "@solana/web3.js";
import wallet from "../wallets/my-wba-wallet.json";

const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

const connection = new Connection(clusterApiUrl("devnet"), "confirmed");

const mintPubkey = new PublicKey(
  "5URbX6zTaujCAKLkjWAYyaHEoyjn9ntZTDr8ipcu7Cni"
);

const decimalsPerToken = 1_000_000n; // decimals: 6

const toPubkey = new PublicKey("CvgLY6kKfrfwcUguxfD9F9XoruNojgcsKRMgzVAfWkSs");

(async () => {
  try {
    const fromAta = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      mintPubkey,
      keypair.publicKey
    );

    const toAta = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      mintPubkey,
      toPubkey
    );

    const sig = await transfer(
      connection,
      keypair,
      fromAta.address,
      toAta.address,
      keypair,
      20n * decimalsPerToken
    );
    console.log(
      `Transferred 20 tokens to ATA at ${toAta.address}. Transaction signature: ${sig}`
    );
  } catch (error) {
    console.error(`Oops, something went wrong: ${error}`);
  }
})();

/*
> Output:
Transferred 20 tokens to ATA at 4KjRk86pSvCuRoTnZhce7QuQvYSzzvoj8VcRsjNaqGyo. Transaction signature: 3c5Z9w5Pfzui7JkCnNoY3YXyAYrESHdmr73X7MtWptscEUz2hEMZECG7QdLwvWJ5PAXN3a6yZVTFCRqZPvyqULbn
*/
