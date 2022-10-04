import { Icon, IconProps } from "./icon";

export interface FieldProps {
  name: string;
  type: string;
  label?: string;
  icon?: IconProps;
  placeholder?: string;
}

export function Field({ name, type, label, icon, placeholder }: FieldProps) {
  return (
    <div className="field">
      {label && (
        <label htmlFor={name} className="label">
          {label}
        </label>
      )}
      <div className={"control" + (icon && " has-icons-left")}>
        {icon && <Icon {...icon} />}
        <input
          name={name}
          type={type}
          placeholder={placeholder}
          className="input"
        />
      </div>
    </div>
  );
}
