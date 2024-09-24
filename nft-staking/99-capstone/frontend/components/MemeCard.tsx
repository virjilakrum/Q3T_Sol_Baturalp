import cn from "@/utils/classNamesHelper";

interface Props {
  className?: string;
}

export default function MemeCard({ className }: Props) {
  return (
    <div
      className={cn(
        "bg-slate-800 rounded-lg p-2 flex flex-col gap-2",
        className
      )}
    >
      <div className="flex justify-between items-center">
        <div className="flex items-center gap-2">
          <div className="w-8 h-8 bg-red-400 rounded-full"></div>
          <div>John Doe</div>
        </div>
        <div>2d</div>
      </div>
      <img src="https://picsum.photos/200" alt="" className="w-full" />
      <div className="flex justify-between">
        <div className="flex">asd</div>
        <div className="flex">fgh</div>
      </div>
    </div>
  );
}
