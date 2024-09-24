import { getOrCreateAssociatedTokenAccount, mintTo } from "@solana/spl-token";
import { Connection, Keypair, PublicKey, clusterApiUrl } from "@solana/web3.js";
import wallet from "../wallets/my-wba-wallet.json";

const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

const connection = new Connection(clusterApiUrl("devnet"), "confirmed");

const mintPubkey = new PublicKey(
  "5URbX6zTaujCAKLkjWAYyaHEoyjn9ntZTDr8ipcu7Cni"
);

const decimalsPerToken = 1_000_000n; // decimals: 6

(async () => {
  try {
    const ata = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      mintPubkey,
      keypair.publicKey
    );
    console.log(`Your ATA address is: ${ata.address.toBase58()}`);

    const sig = await mintTo(
      connection,
      keypair,
      mintPubkey,
      ata.address,
      keypair,
      100n * decimalsPerToken
    );
    console.log(`Minted 100 tokens to your ATA. Transaction signature: ${sig}`);
  } catch (error) {
    console.error(`Oops, something went wrong: ${error}`);
  }
})();

/*
> Output:
Your ATA address is: 9VBbBro4BeoksiR8mkn6JKYS3tGf3heDuxn3niSFqeYF
Minted 100 tokens to your ATA. Transaction signature: 54oscDjWzGMTZPiYoZJaqSbj5Xt2MNtj5VoKeW7KRJUktrPc1XRpbFBfpGgzoUh1xVTPwB1w7i6hkQAMEikZXAoC
*/
