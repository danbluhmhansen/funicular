import { createRoot } from "react-dom/client";

const fields = [
  { name: "Email", type: "email", placeholder: "Email" },
  { name: "Password", type: "password", placeholder: "Password" },
  { name: "ConfirmPassword", type: "password", placeholder: "Password" },
];

export default function Register() {
  return (
    <>
      <h2 className="title">Register</h2>
      <form method="post" action="/account/register?returnUrl=/">
        <h4 className="title">Create a new account.</h4>
        <input
          type="hidden"
          name="__RequestVerificationToken"
          value={globalThis.antiforgeryToken}
        />
        {fields.map(({ name, type, placeholder }) => (
          <div key={name} className="field">
            <label htmlFor={name} className="label">
              {name}
            </label>
            <div className="control has-icons-left has-icons-right">
              <input
                name={name}
                type={type}
                placeholder={placeholder}
                className="input"
              />
            </div>
          </div>
        ))}
        <div className="control">
          <input type="submit" value="Register" className="button is-link" />
        </div>
      </form>
    </>
  );
}

createRoot(document.querySelector("#container") as HTMLElement).render(
  <Register />
);
