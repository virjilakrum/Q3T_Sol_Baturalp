import cn from "@/utils/classNamesHelper";
import JustX from "./JustX";
import styles from "./LogoAnimated.module.css";

interface Props {
  height: number;
  className?: string;
  style?: React.CSSProperties;
}

export default function LogoAnimated({ height, className, style }: Props) {
  return (
    <div className={cn("relative w-fit", className)} style={style}>
      <JustX height={height} className={styles.zero} />
      <JustX height={height} className={styles.one} />
      <JustX height={height} className={styles.two} />
    </div>
  );
}
