export default interface ManageIndex {
  hasPassword: boolean;
  phoneNumber: string | undefined;
  twoFactor: boolean;
  browserRemebered: boolean;
}
