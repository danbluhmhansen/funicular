import { createRoot } from "react-dom/client";

interface Provider {
  name: string;
  display: string;
}

const fields = [
  { name: "Email", type: "email", placeholder: "Email" },
  { name: "Password", type: "password", placeholder: "Password" },
];

const providers: Provider[] | undefined = undefined;

export default function Login() {
  return (
    <>
      <h2 className="title">Log in</h2>
      <div className="columns">
        <div className="column">
          <form method="post" action="/account/login?returnUrl=/">
            <h4 className="title">Use a local account to log in.</h4>
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
            <div className="field">
              <div className="control">
                <label className="checkbox">
                  <input name="RememberMe" type="checkbox" /> Remember me?
                </label>
              </div>
            </div>
            <div className="control">
              <input type="submit" value="Log in" className="button is-link" />
            </div>
            <p>
              <a href="/account/register?returnUrl=/">
                Register as a new user?
              </a>
            </p>
            <p>
              <a href="/account/forgetpassword">Forgot your password?</a>
            </p>
          </form>
        </div>
        <div className="column">
          <h4 className="title">Use another service to log in.</h4>
          {providers ? (
            <form method="post" action="/account/externallogin?returnUrl=/">
              {providers.map((p) => (
                <input
                  type="submit"
                  name="provider"
                  value={p.name}
                  className="button is-link"
                />
              ))}
            </form>
          ) : (
            <p>
              There are no external authentication services configured. See{" "}
              <a href="http://go.microsoft.com/fwlink/?LinkID=532715">
                this article
              </a>{" "}
              for details on setting up this ASP.NET application to support
              logging in via external services.
            </p>
          )}
        </div>
      </div>
    </>
  );
}

createRoot(document.querySelector("#container") as HTMLElement).render(
  <Login />
);
