namespace Funicular.Client;

using Microsoft.AspNetCore.Components;
using Microsoft.AspNetCore.Components.WebAssembly.Authentication;
using Microsoft.JSInterop;

// RemoteAuthenticatorView only uses the path and query part of a URI when redirecting,
// so it cannot redirect to an app hosted on another base URI/port.
// https://github.com/dotnet/aspnetcore/issues/25153
public class CustomRemoteAuthenticatorView : RemoteAuthenticatorView
{
    [Inject]
    internal IJSRuntime Js { get; set; } = null!;

    [Inject]
    internal NavigationManager Navigation { get; set; } = null!;

    protected override async Task OnParametersSetAsync()
    {
        switch (Action)
        {
            case RemoteAuthenticationActions.Profile when ApplicationPaths.RemoteProfilePath != null:
                UserProfile ??= LoggingIn;
                await RedirectToProfile();
                break;
            case RemoteAuthenticationActions.Register when ApplicationPaths.RemoteRegisterPath != null:
                Registering ??= LoggingIn;
                await RedirectToRegister();
                break;
            default:
                await base.OnParametersSetAsync();
                break;
        }
    }

    private ValueTask RedirectToProfile() =>
        Js.InvokeVoidAsync("location.replace", Navigation.ToAbsoluteUri(ApplicationPaths.RemoteProfilePath));

    private ValueTask RedirectToRegister()
    {
        Uri loginUrl = Navigation.ToAbsoluteUri(ApplicationPaths.LogInPath);
        Uri registerUrl = Navigation.ToAbsoluteUri($"{ApplicationPaths.RemoteRegisterPath}?returnUrl={loginUrl}");

        return Js.InvokeVoidAsync("location.replace", registerUrl);
    }
}
