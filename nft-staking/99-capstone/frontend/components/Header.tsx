"use client";

import cn from "@/utils/classNamesHelper";
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";
import { useEffect, useState } from "react";
import styles from "./Header.module.css";
import LogoFull from "./LogoFull";

export default function Header() {
  // Don't render the wallet button on the server to avoid hydration error
  // Thanks Next.js, I hate this
  const [isClient, setIsClient] = useState(false);
  useEffect(() => {
    setIsClient(true);
  }, []);

  return (
    <header>
      <div
        className={cn(styles.fadeToMemes, "fixed top-0 left-0 right-0 h-24")}
      ></div>

      <div className="fixed top-0 left-0 right-0 h-20 flex justify-center items-center ">
        <LogoFull height={60} />
      </div>

      <div className="fixed top-0 left-0 right-0 h-20 flex justify-between items-center px-6">
        <div className="flex items-center gap-3">
          <div className="bg-red-800 text-white rounded-lg px-6 h-12 flex items-center">
            + Create
          </div>
        </div>
        <div className="flex items-center gap-3">
          {isClient && <WalletMultiButton />}
        </div>
      </div>
    </header>
  );
}
