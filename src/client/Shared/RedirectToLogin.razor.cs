namespace Funicular.Client;

using Microsoft.AspNetCore.Components;

public partial class RedirectToLogin : ComponentBase
{
    [Inject]
    private NavigationManager? Navigation { get; init; }

    protected override void OnInitialized() =>
        Navigation?.NavigateTo($"authentication/login?returnUrl={Uri.EscapeDataString(Navigation.Uri)}");
}
