import { Title } from "@funicular/shared";
import Page from "page";
import type ManageIndex from "../../models/manage";

const model: ManageIndex = globalThis.model;

export default function Index() {
  return (
    <>
      <Title size={2}>Manage your account</Title>
      <Title size={4} sub>
        Change your account settings
      </Title>
      <table className="table">
        <tbody>
          <tr>
            <td className="has-text-right">Password:</td>
            <td>
              {model.hasPassword ? (
                <a
                  href="/manage/changepassword"
                  className="button is-link is-small"
                >
                  Change
                </a>
              ) : (
                <a
                  href="/manage/setpassword"
                  className="button is-link is-small"
                >
                  Create
                </a>
              )}
            </td>
          </tr>
          <tr>
            <td className="has-text-right">External Logins:</td>
            <td>
              <a
                href="/manage/managelogins"
                className="button is-link is-small"
              >
                Manage
              </a>
            </td>
          </tr>
          <tr>
            <td className="has-text-right">Phone Number:</td>
            <td>
              {model.phoneNumber && model.phoneNumber !== "" ? (
                <>
                  <a
                    href="/manage/addphonenumber"
                    className="button is-link is-small"
                  >
                    Change
                  </a>
                  <a
                    href="/manage/removephonenumber"
                    className="button is-link is-small"
                  >
                    Remove
                  </a>
                </>
              ) : (
                <a
                  href="/manage/addphonenumber"
                  className="button is-link is-small"
                >
                  Add
                </a>
              )}
            </td>
          </tr>
          <tr>
            <td className="has-text-right">Two-Factor Authentication:</td>
            <td>
              {model.twoFactor ? (
                <form
                  method="post"
                  action="/manage/disabletwofactorauthentication"
                >
                  <input
                    type="submit"
                    value="Disable"
                    className="button is-link is-small"
                  />
                </form>
              ) : (
                <form
                  method="post"
                  action="/manage/enabletwofactorauthentication"
                >
                  <input
                    type="submit"
                    value="Enable"
                    className="button is-link is-small"
                  />
                </form>
              )}
            </td>
          </tr>
        </tbody>
      </table>
    </>
  );
}

Page(<Index />);
