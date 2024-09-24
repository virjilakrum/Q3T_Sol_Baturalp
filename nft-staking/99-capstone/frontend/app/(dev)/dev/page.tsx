"use client";

import MemeGrid from "@/components/MemeGrid";
import useRemixers from "@/programs/useRemixers";
import { useStore } from "@/store";
import { useEffect } from "react";

export default function Home() {
  const remixers = useRemixers();
  const setIsShowSpinner = useStore((state) => state.setIsShowSpinner);

  useEffect(() => {
    const timeout = setTimeout(() => {
      setIsShowSpinner(false);
    }, 1000);
    return () => {
      clearTimeout(timeout);
    };
  }, []);

  const initialize = async () => {
    if (!remixers) return;
    const sig = await remixers.methods.initialize().rpc();
    console.log(`✅ Success! Sig: ${sig}`);
  };

  const createMeme = async () => {
    if (!remixers) return;
    const seed = 123;
    const sig = await remixers.methods.createMeme(seed).rpc();
    console.log(`✅ Success! Sig: ${sig}`);
  };

  const fetchMemes = async () => {
    if (!remixers) return;
    const foo = await remixers.account.meme.all();
    console.log(foo);
  };

  return (
    <>
      <MemeGrid className="max-w-[900px] mx-auto" />

      <button type="button" onClick={() => initialize()}>
        initialize
      </button>
      <button type="button" onClick={() => createMeme()}>
        create meme
      </button>
      <button type="button" onClick={() => fetchMemes()}>
        fetch memes
      </button>
    </>
  );
}
