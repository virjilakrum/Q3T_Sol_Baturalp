"use client";

import { useStore } from "@/store";
import cn from "@/utils/classNamesHelper";
import LogoAnimated from "./LogoAnimated";

export default function Spinner() {
  const isShowSpinner = useStore((state) => state.isShowSpinner);

  return (
    <div
      className={cn(
        "fixed inset-0 z-50 bg-black flex justify-center items-center",
        !isShowSpinner && "hidden"
      )}
    >
      <LogoAnimated height={256} />
    </div>
  );
}
