namespace Funicular.Server.Controllers;

using Microsoft.AspNetCore.Mvc;

using OpenIddict.Validation.AspNetCore;

public class HomeController : Controller
{
    public IActionResult Index() => View();

    public IActionResult About() => View();

    public IActionResult Contact() => View();

    public IActionResult Error() => View("~/Views/Shared/Error.cshtml");

    [Microsoft.AspNetCore.Authorization.Authorize(
        AuthenticationSchemes = OpenIddictValidationAspNetCoreDefaults.AuthenticationScheme
    )]
    [HttpGet("~/test")]
    public ActionResult Test() => Ok(new { test = "Hello, World!" });
}