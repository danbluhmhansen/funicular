namespace Funicular.Server.Controllers;

using Funicular.Server.Data.Models;
using Funicular.Server.Models.Account;

using Microsoft.AspNetCore.Authorization;
using Microsoft.AspNetCore.Identity;
using Microsoft.AspNetCore.Mvc;

[Authorize]
public class AccountController : ControllerBase
{
    private readonly UserManager<FunicularUser> userManager;
    private readonly SignInManager<FunicularUser> signInManager;

    // private readonly IEmailSender _emailSender;
    // private readonly ISmsSender _smsSender;

    public AccountController(UserManager<FunicularUser> userManager, SignInManager<FunicularUser> signInManager)
    // IEmailSender emailSender,
    // ISmsSender smsSender,
    {
        this.userManager = userManager;
        this.signInManager = signInManager;
        // _emailSender = emailSender;
        // _smsSender = smsSender;
    }

    [HttpPost]
    [AllowAnonymous]
    [ValidateAntiForgeryToken]
    public async Task<IActionResult> Login(Login model, string returnUrl = null)
    {
        if (!ModelState.IsValid)
            return BadRequest(model);

        var result = await signInManager.PasswordSignInAsync(model.Email, model.Password, model.RememberMe, false);
        if (result.Succeeded)
            return RedirectToLocal(returnUrl);
        // if (result.RequiresTwoFactor)
        // return RedirectToAction(nameof(SendCode), new { ReturnUrl = returnUrl, RememberMe = model.RememberMe });
        if (result.IsLockedOut)
            return BadRequest("Lockout");

        ModelState.AddModelError(string.Empty, "Invalid login attempt.");
        return BadRequest(model);
    }

    [HttpPost]
    [AllowAnonymous]
    [ValidateAntiForgeryToken]
    public async Task<IActionResult> Register(Register model, string returnUrl = null)
    {
        if (!ModelState.IsValid)
            return BadRequest(model);

        var user = new FunicularUser { UserName = model.Email, Email = model.Email };
        var result = await userManager.CreateAsync(user, model.Password);

        if (!result.Succeeded)
        {
            AddErrors(result);
            return BadRequest(model);
        }

        // For more information on how to enable account confirmation and password reset please visit http://go.microsoft.com/fwlink/?LinkID=532713
        // Send an email with this link
        //var code = await _userManager.GenerateEmailConfirmationTokenAsync(user);
        //var callbackUrl = Url.Action("ConfirmEmail", "Account", new { userId = user.Id, code = code }, protocol: Context.Request.Scheme);
        //await _emailSender.SendEmailAsync(model.Email, "Confirm your account",
        //    "Please confirm your account by clicking this link: <a href=\"" + callbackUrl + "\">link</a>");
        await signInManager.SignInAsync(user, isPersistent: false);
        return RedirectToLocal(returnUrl);
    }

    [HttpPost]
    [ValidateAntiForgeryToken]
    public async Task<IActionResult> LogOff()
    {
        await signInManager.SignOutAsync();
        return Redirect("/");
    }

    private void AddErrors(IdentityResult result)
    {
        foreach (var error in result.Errors)
        {
            ModelState.AddModelError(string.Empty, error.Description);
        }
    }

    private IActionResult RedirectToLocal(string returnUrl) => Redirect(Url.IsLocalUrl(returnUrl) ? returnUrl : "/");
}