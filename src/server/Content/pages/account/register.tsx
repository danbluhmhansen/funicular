import { Field, FieldProps, Submit, Title } from "@funicular/shared";
import AntiforgeryToken from "components/antiforgery-token";
import Page from "page";

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
  {
    name: "confirmPassword",
    type: "password",
    label: "Confirm password",
    icon: {
      icon: "lock",
      size: "small",
    },
    placeholder: "Password",
  },
];

export default function Register() {
  return (
    <>
      <Title size={2}>Register</Title>
      <form method="post" action="/account/register">
        <Title size={4}>Create a new account.</Title>
        <AntiforgeryToken />
        <input type="hidden" name="returnUrl" value="/" />
        {fields.map((field) => (
          <Field key={field.name} {...field} />
        ))}
        <Submit value="Register" />
      </form>
    </>
  );
}

Page(<Register />);
