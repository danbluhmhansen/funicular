import { Field, FieldProps, Submit, Title } from "@funicular/shared";
import AntiforgeryToken from "components/antiforgery-token";
import Page from "page";

interface Provider {
  name: string;
  display: string;
}

const fields: FieldProps[] = [
  {
    name: "email",
    type: "email",
    label: "Email",
    icon: {
      icon: "mail",
      size: "small",
    },
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
    placeholder: "Password",
  },
];

const providers: Provider[] | undefined = undefined;

export default function Login() {
  return (
    <>
      <Title size={2}>Log in</Title>
      <div className="columns">
        <div className="column">
          <form method="post" action="/account/login">
            <Title size={4}>Use a local account to log in.</Title>
            <AntiforgeryToken />
            <input type="hidden" name="returnUrl" value="/" />
            {fields.map((field) => (
              <Field key={field.name} {...field} />
            ))}
            <div className="field">
              <div className="control">
                <label className="checkbox">
                  <input name="RememberMe" type="checkbox" /> Remember me?
                </label>
              </div>
            </div>
            <Submit value="Log in" />
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
          <Title size={4}>Use another service to log in.</Title>
          {providers ? (
            <form method="post" action="/account/externallogin">
              <input type="hidden" name="returnUrl" value="/" />
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
