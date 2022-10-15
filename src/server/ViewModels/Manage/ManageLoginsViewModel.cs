namespace Funicular.Server.ViewModels.Manage;

using Microsoft.AspNetCore.Authentication;
using Microsoft.AspNetCore.Identity;

public class ManageLoginsViewModel
{
    public IList<UserLoginInfo>? CurrentLogins { get; set; }

    public IList<AuthenticationScheme>? OtherLogins { get; set; }
}