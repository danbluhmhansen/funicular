namespace Funicular.Client.Shared;

using Microsoft.AspNetCore.Components;
using Microsoft.AspNetCore.Components.WebAssembly.Authentication;

public partial class NavMenu
{
    [Inject]
    public SignOutSessionStateManager? SignOutManager { get; set; }

    [Inject]
    public NavigationManager? Navigation { get; set; }

    private bool active = false;

    private void ToggleNavMenu() => active = !active;

    private async Task BeginSignOutAsync()
    {
        if (SignOutManager is not null)
            await SignOutManager.SetSignOutState();
        Navigation?.NavigateTo("authentication/logout");
    }
}
