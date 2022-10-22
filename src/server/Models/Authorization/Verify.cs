namespace Funicular.Server.Models.Authorization;

using System.ComponentModel.DataAnnotations;

using Microsoft.AspNetCore.Mvc;
using Microsoft.AspNetCore.Mvc.ModelBinding;

using static OpenIddict.Abstractions.OpenIddictConstants.Parameters;

public record Verify(
    [property: Display(Name = "Application")] string ApplicationName,
    [property: BindNever, Display(Name = "Error")] string Error,
    [property: BindNever, Display(Name = "Error description")] string ErrorDescription,
    string Scope,
    [property: FromQuery(Name = UserCode), Display(Name = "User code")] string UserCode
)
{
    public Verify() : this(string.Empty, string.Empty, string.Empty, string.Empty, string.Empty) { }

    public Verify(string applicationName, string scope, string userCode)
        : this(applicationName, string.Empty, string.Empty, scope, userCode) { }

    public Verify(string error, string errorDescription)
        : this(string.Empty, error, errorDescription, string.Empty, string.Empty) { }
}