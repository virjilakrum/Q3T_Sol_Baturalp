interface Props {
  width?: number;
  height?: number;
  style?: React.CSSProperties;
  className?: string;
}

export default function JustX({ width, height, style, className }: Props) {
  return (
    <svg
      width={width}
      height={height}
      style={style}
      className={className}
      viewBox="0 0 467.67 512"
      xmlns="http://www.w3.org/2000/svg"
    >
      <g transform="translate(-22.164)">
        <path
          d="m22.164 512 206.86-295.17v67.694l-192.43-284.52h103.77l63.227 94.84q10.996 16.838 21.648 33.675 10.652 16.494 21.305 33.332t20.961 33.675h-21.648q10.996-16.838 21.648-33.675t21.648-33.332q10.996-16.838 21.992-33.675l64.258-94.84h101.03l-190.02 280.05v-64.945l203.43 296.89h-105.49l-78.346-116.14q-9.6215-14.776-19.587-29.208-9.6215-14.776-19.243-29.208-9.2779-14.776-18.899-29.552h15.463q-9.2779 14.776-19.243 29.552-9.6215 14.432-19.587 29.208-9.6215 14.432-19.93 29.208l-79.721 116.14z"
          fill="currentColor"
        />
      </g>
    </svg>
  );
}
