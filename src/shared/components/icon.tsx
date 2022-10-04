export type Color =
  | "primary"
  | "link"
  | "info"
  | "success"
  | "warning"
  | "danger"
  | undefined;

export interface IconProps {
  icon: "mail" | "lock";
  size?: "small" | "medium" | "large" | undefined;
  color?: Color;
}

export function Icon({ icon, size, color }: IconProps) {
  return (
    <span
      className={
        "icon" + (size && ` is-${size}`) + (color && ` has-text-${color}`)
      }
    >
      <i className={`ti ti-${icon}`} />
    </span>
  );
}

export function IconText({
  text,
  color,
  icon,
}: {
  text: string;
  color?: Color;
  icon: IconProps;
}) {
  return (
    <span className={"icon-text" + (color && ` has-text-${color}`)}>
      <Icon {...icon} color={color ? undefined : icon.color} />
      <span>{text}</span>
    </span>
  );
}
