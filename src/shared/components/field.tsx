import { Icon, IconProps } from "./icon";

export interface FieldProps {
  name: string;
  type: "email" | "password" | "checkbox";
  label?: string;
  icon?: IconProps;
  defaultValue?: string | number | readonly string[] | undefined;
  placeholder?: string;
}

export function Field({
  name,
  type,
  label,
  icon,
  defaultValue,
  placeholder,
}: FieldProps) {
  return (
    <div className="field">
      {type !== "checkbox" && label && (
        <label htmlFor={name} className="label">
          {label}
        </label>
      )}
      <div className={"control" + (icon && " has-icons-left")}>
        {icon && <Icon {...icon} />}
        {type === "checkbox" && label ? (
          <label className="checkbox">
            <input name={name} type="checkbox" defaultValue={defaultValue} />{" "}
            {label}
          </label>
        ) : (
          <input
            name={name}
            type={type}
            defaultValue={defaultValue}
            placeholder={placeholder}
            className="input"
          />
        )}
      </div>
    </div>
  );
}
