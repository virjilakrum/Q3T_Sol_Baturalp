import cn from "@/utils/classNamesHelper";
import MemeCard from "./MemeCard";

interface Props {
  className?: string;
}

export default function MemeGrid({ className }: Props) {
  return (
    <div className={cn("grid grid-cols-3 gap-4", className)}>
      <MemeCard />
      <MemeCard />
      <MemeCard />
      <MemeCard />
      <MemeCard />
      <MemeCard />
      <MemeCard />
      <MemeCard />
    </div>
  );
}
