import LogoAnimated from "@/components/LogoAnimated";

export default function ComingSoon() {
  return (
    <main className="flex flex-col justify-center items-center text-center px-4 h-full">
      <LogoAnimated height={256} className="mb-16" />
      <p className="text-2xl mb-4">
        Get ready for the next era of{" "}
        <span className="font-[600]">The Internet Culture</span>.
      </p>
      <p className="text-sm">September 27th, 2024</p>
    </main>
  );
}
