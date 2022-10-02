namespace Funicular.Server.Controllers;

using Microsoft.AspNetCore.Mvc;

public class HomeController : Controller
{
    public IActionResult Index() => View();

    public IActionResult About() => View();

    public IActionResult Contact() => View();

    public IActionResult Error() => View("~/Views/Shared/Error.cshtml");
}