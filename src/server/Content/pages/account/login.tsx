import { Field, FieldProps, Submit, Title } from "@funicular/shared";
import AntiforgeryToken from "components/antiforgery-token";
import type AccountLogin from "models/account/login";
import Page from "page";

interface Provider {
  name: string;
  display: string;
}

const model: AccountLogin = globalThis.model;

const fields: FieldProps[] = [
  {
    name: "email",
    type: "email",
    label: "Email",
    icon: {
      icon: "mail",
      size: "small",
    },
    defaultValue: model.email,
    placeholder: "Email",
  },
  {
    name: "password",
    type: "password",
    label: "Password",
    icon: {
      icon: "lock",
      size: "small",
    },
    defaultValue: model.password,
    placeholder: "Password",
  },
  {
    name: "rememberMe",
    type: "checkbox",
    label: "Remember me?",
    defaultValue: model.rememberMe + "",
  },
];

const providers: Provider[] | undefined = undefined;

export default function Login() {
  const params = new URLSearchParams(window.location.search);
  const returnUrl = params.get("returnUrl") ?? "/manage";
  return (
    <>
      <Title size={2}>Log in</Title>
      <div className="columns">
        <div className="column">
          <form method="post" action="/account/login">
            <Title size={4}>Use a local account to log in.</Title>
            <AntiforgeryToken />
            <input type="hidden" name="returnUrl" value={returnUrl} />
            {fields.map((field) => (
              <Field key={field.name} {...field} />
            ))}
            <Submit value="Log in" />
            <p>
              <a href={"/account/register?returnUrl=" + returnUrl}>
                Register as a new user?
              </a>
            </p>
            <p>
              <a href="/account/forgetpassword">Forgot your password?</a>
            </p>
          </form>
        </div>
        <div className="column">
          <Title size={4}>Use another service to log in.</Title>
          {providers ? (
            <form method="post" action="/account/externallogin">
              <input type="hidden" name="returnUrl" value={returnUrl} />
              {providers.map(({ name }) => (
                <Submit value={name} name="provider" />
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

Page(<Login />);
