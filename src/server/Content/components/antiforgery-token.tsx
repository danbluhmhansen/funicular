export default function AntiforgeryToken() {
  return (
    <input
      type="hidden"
      name="__RequestVerificationtoken"
      value={globalThis.antiforgeryToken}
    />
  );
}
