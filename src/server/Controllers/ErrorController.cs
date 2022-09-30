namespace Funicular.Server.Controllers;

using Funicular.Server.ViewModels.Shared;

using Microsoft.AspNetCore;
using Microsoft.AspNetCore.Mvc;

public class ErrorController : Controller
{
    [HttpGet, HttpPost, Route("~/error")]
    public IActionResult Error()
    {
        // If the error was not caused by an invalid
        // OIDC request, display a generic error page.
        var response = HttpContext.GetOpenIddictServerResponse();
        return response is null
            ? View(new ErrorViewModel())
            : View(new ErrorViewModel { Error = response.Error, ErrorDescription = response.ErrorDescription });
    }
}