import { AnchorProvider, Program, Wallet } from "@coral-xyz/anchor";
import { Connection, Keypair } from "@solana/web3.js";
import wallet from "../../wallets/dev-wallet.json";
import RemixersIdl from "./anchor-artifacts/idl/remixers.json";
import { Remixers } from "./anchor-artifacts/types/remixers";

const connection = new Connection("http://127.0.0.1:8899");
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
const provider = new AnchorProvider(connection, new Wallet(keypair), {
  commitment: "confirmed",
});

const program = new Program(RemixersIdl as Remixers, provider);

async function main() {
  try {
    const sig = await program.methods.initialize().rpc();
    console.log("Success! Check 'solana logs'.");
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
}
main();
