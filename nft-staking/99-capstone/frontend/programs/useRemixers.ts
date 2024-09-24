import {
  AnchorProvider,
  getProvider,
  Program,
  setProvider,
} from "@coral-xyz/anchor";
import { useAnchorWallet, useConnection } from "@solana/wallet-adapter-react";
import { useEffect, useState } from "react";
import idl from "../anchor-artifacts/idl/remixers.json";
import { Remixers } from "../anchor-artifacts/types/remixers";

export default function useRemixers() {
  // const { connection } = useConnection();
  // let wallet = null;
  // try {
  //   wallet = useAnchorWallet();
  // } catch (error: any) {
  //   return null;
  // }

  // if (wallet) {
  //   const provider = new AnchorProvider(connection, wallet, {
  //     commitment: "confirmed",
  //   });
  //   return new Program(idl as Remixers, provider);
  // } else {
  //   return null;
  // }

  const { connection } = useConnection();
  const wallet = useAnchorWallet();

  const [program, setProgram] = useState<Program<Remixers>>();

  useEffect(() => {
    let provider;
    try {
      provider = getProvider();
    } catch {
      if (!wallet) return;
      provider = new AnchorProvider(connection, wallet, {
        commitment: "confirmed",
      });
      setProvider(provider);
    }

    const program = new Program(idl as Remixers);
    setProgram(program);
  }, [connection, wallet]);

  return program;
}
