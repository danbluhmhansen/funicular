export function Submit({
  value,
  name,
}: {
  value: string | number | readonly string[];
  name?: string | undefined;
}) {
  return (
    <div className="control">
      <input
        type="submit"
        value={value}
        name={name}
        className="button is-link"
      />
    </div>
  );
}
