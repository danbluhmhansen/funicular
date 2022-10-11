namespace Funicular.Server.Controllers;

using System.Security.Claims;

using FluentValidation;

using Funicular.Server.Data.Models;
using Funicular.Server.Extensions;
using Funicular.Server.ViewModels.Account;

using Microsoft.AspNetCore.Authorization;
using Microsoft.AspNetCore.Identity;
using Microsoft.AspNetCore.Mvc;
using Microsoft.AspNetCore.Mvc.Rendering;

/// <summary>Account controller</summary>
[Authorize]
public class AccountController : Controller
{
    private readonly UserManager<FunicularUser> userManager;
    private readonly SignInManager<FunicularUser> signInManager;

    // private readonly IEmailSender _emailSender;
    // private readonly ISmsSender _smsSender;

    /// <summary>Constructor</summary>
    public AccountController(UserManager<FunicularUser> userManager, SignInManager<FunicularUser> signInManager)
    {
        this.userManager = userManager;
        this.signInManager = signInManager;
        // _emailSender = emailSender;
        // _smsSender = smsSender;
    }

    /// <summary>GET: /Account/Login</summary>
    [HttpGet]
    [AllowAnonymous]
    public IActionResult Login(string? returnUrl = default)
    {
        ViewData["ReturnUrl"] = returnUrl;
        return View();
    }

    /// <summary>POST: /Account/Login</summary>
    [HttpPost]
    [AllowAnonymous]
    [ValidateAntiForgeryToken]
    public async Task<IActionResult> Login(
        LoginViewModel model,
        [FromServices] IValidator<LoginViewModel> validator,
        string returnUrl = default!
    )
    {
        ViewData["ReturnUrl"] = returnUrl;

        var validationResult = await validator.ValidateAsync(model);
        if (!validationResult.IsValid)
        {
            validationResult.AddToModelState(ModelState);
            return View(model);
        }

        // This doesn't count login failures towards account lockout
        // To enable password failures to trigger account lockout, set lockoutOnFailure: true
        var result = await signInManager.PasswordSignInAsync(
            model.Email,
            model.Password,
            model.RememberMe,
            lockoutOnFailure: false
        );
        if (result.Succeeded)
            return RedirectToLocal(returnUrl);
        if (result.RequiresTwoFactor)
            return RedirectToAction(nameof(SendCode), new { ReturnUrl = returnUrl, model.RememberMe });
        if (result.IsLockedOut)
            return View("Lockout");
        else
        {
            ModelState.AddModelError(string.Empty, "Invalid login attempt.");
            return View(model);
        }
    }

    ///<summary>GET: /Account/Register</summary>
    [HttpGet]
    [AllowAnonymous]
    public IActionResult Register(string? returnUrl = default)
    {
        ViewData["ReturnUrl"] = returnUrl;
        return View();
    }

    ///<summary>POST: /Account/Register</summary>
    [HttpPost]
    [AllowAnonymous]
    [ValidateAntiForgeryToken]
    public async Task<IActionResult> Register(
        RegisterViewModel model,
        [FromServices] IValidator<RegisterViewModel> validator,
        string returnUrl = default!
    )
    {
        ViewData["ReturnUrl"] = returnUrl;

        var validationResult = await validator.ValidateAsync(model);
        if (!validationResult.IsValid)
        {
            validationResult.AddToModelState(ModelState);
            return View(model);
        }

        var user = new FunicularUser { UserName = model.Email, Email = model.Email };
        var result = await userManager.CreateAsync(user, model.Password);
        if (result.Succeeded)
        {
            // For more information on how to enable account confirmation and password reset please visit http://go.microsoft.com/fwlink/?LinkID=532713
            // Send an email with this link
            //var code = await _userManager.GenerateEmailConfirmationTokenAsync(user);
            //var callbackUrl = Url.Action("ConfirmEmail", "Account", new { userId = user.Id, code = code }, protocol: Context.Request.Scheme);
            //await _emailSender.SendEmailAsync(model.Email, "Confirm your account",
            //    "Please confirm your account by clicking this link: <a href=\"" + callbackUrl + "\">link</a>");
            await signInManager.SignInAsync(user, isPersistent: false);
            return RedirectToLocal(returnUrl);
        }

        AddErrors(result);
        // If we got this far, something failed, redisplay form
        return View(model);
    }

    ///<summary>POST: /Account/LogOff</summary>
    [HttpPost]
    [ValidateAntiForgeryToken]
    public async Task<IActionResult> LogOff()
    {
        await signInManager.SignOutAsync();
        return RedirectToAction(nameof(HomeController.Index), "Home");
    }

    ///<summary>POST: /Account/ExternalLogin</summary>
    [HttpPost]
    [AllowAnonymous]
    [ValidateAntiForgeryToken]
    public IActionResult ExternalLogin(string provider, string returnUrl = default!)
    {
        returnUrl ??= string.Empty;
        // Request a redirect to the external login provider.
        var redirectUrl = Url.Action("ExternalLoginCallback", "Account", new { ReturnUrl = returnUrl });
        var properties = signInManager.ConfigureExternalAuthenticationProperties(provider, redirectUrl);
        return new ChallengeResult(provider, properties);
    }

    ///<summary>GET: /Account/ExternalLoginCallback</summary>
    [HttpGet]
    [AllowAnonymous]
    public async Task<IActionResult> ExternalLoginCallback(string returnUrl = default!)
    {
        returnUrl ??= string.Empty;
        var info = await signInManager.GetExternalLoginInfoAsync();
        if (info is null)
            return RedirectToAction(nameof(Login));

        // Sign in the user with this external login provider if the user already has a login.
        var result = await signInManager.ExternalLoginSignInAsync(
            info.LoginProvider,
            info.ProviderKey,
            isPersistent: false
        );
        if (result.Succeeded)
            return RedirectToLocal(returnUrl);
        if (result.RequiresTwoFactor)
            return RedirectToAction(nameof(SendCode), new { ReturnUrl = returnUrl });
        if (result.IsLockedOut)
            return View("Lockout");
        else
        {
            // If the user does not have an account, then ask the user to create an account.
            ViewData["ReturnUrl"] = returnUrl;
            ViewData["LoginProvider"] = info.LoginProvider;
            var email = info.Principal.FindFirstValue(ClaimTypes.Email);
            return View("ExternalLoginConfirmation", new ExternalLoginConfirmationViewModel { Email = email });
        }
    }

    ///<summary>POST: /Account/ExternalLoginConfirmation</summary>
    [HttpPost]
    [AllowAnonymous]
    [ValidateAntiForgeryToken]
    public async Task<IActionResult> ExternalLoginConfirmation(
        ExternalLoginConfirmationViewModel model,
        string returnUrl = default!
    )
    {
        returnUrl ??= string.Empty;
        if (ModelState.IsValid)
        {
            // Get the information about the user from the external login provider
            var info = await signInManager.GetExternalLoginInfoAsync();
            if (info is null)
                return View("ExternalLoginFailure");
            var user = new FunicularUser { UserName = model.Email, Email = model.Email };
            var result = await userManager.CreateAsync(user);
            if (result.Succeeded)
            {
                result = await userManager.AddLoginAsync(user, info);
                if (result.Succeeded)
                {
                    await signInManager.SignInAsync(user, isPersistent: false);
                    return RedirectToLocal(returnUrl);
                }
            }
            AddErrors(result);
        }

        ViewData["ReturnUrl"] = returnUrl;
        return View(model);
    }

    ///<summary>GET: /Account/ConfirmEmail</summary>
    [HttpGet]
    [AllowAnonymous]
    public async Task<IActionResult> ConfirmEmail(string userId, string code)
    {
        if (userId is null || code is null)
            return View("Error");
        var user = await userManager.FindByIdAsync(userId);
        if (user is null)
            return View("Error");
        var result = await userManager.ConfirmEmailAsync(user, code);
        return View(result.Succeeded ? "ConfirmEmail" : "Error");
    }

    ///<summary>GET: /Account/ForgotPassword</summary>
    [HttpGet]
    [AllowAnonymous]
    public IActionResult ForgotPassword() => View();

    ///<summary>POST: /Account/ForgotPassword</summary>
    [HttpPost]
    [AllowAnonymous]
    [ValidateAntiForgeryToken]
    public async Task<IActionResult> ForgotPassword(ForgotPasswordViewModel model)
    {
        if (ModelState.IsValid)
        {
            var user = await userManager.FindByNameAsync(model.Email);
            if (user is null || !await userManager.IsEmailConfirmedAsync(user))
                // Don't reveal that the user does not exist or is not confirmed
                return View("ForgotPasswordConfirmation");

            // For more information on how to enable account confirmation and password reset please visit http://go.microsoft.com/fwlink/?LinkID=532713
            // Send an email with this link
            //var code = await _userManager.GeneratePasswordResetTokenAsync(user);
            //var callbackUrl = Url.Action("ResetPassword", "Account", new { userId = user.Id, code = code }, protocol: Context.Request.Scheme);
            //await _emailSender.SendEmailAsync(model.Email, "Reset Password",
            //   "Please reset your password by clicking here: <a href=\"" + callbackUrl + "\">link</a>");
            //return View("ForgotPasswordConfirmation");
        }

        // If we got this far, something failed, redisplay form
        return View(model);
    }

    ///<summary>GET: /Account/ForgotPasswordConfirmation</summary>
    [HttpGet]
    [AllowAnonymous]
    public IActionResult ForgotPasswordConfirmation() => View();

    ///<summary>GET: /Account/ResetPassword</summary>
    [HttpGet]
    [AllowAnonymous]
    public IActionResult ResetPassword(string? code = null) => code is null ? View("Error") : View();

    ///<summary>POST: /Account/ResetPassword</summary>
    [HttpPost]
    [AllowAnonymous]
    [ValidateAntiForgeryToken]
    public async Task<IActionResult> ResetPassword(ResetPasswordViewModel model)
    {
        if (!ModelState.IsValid)
            return View(model);
        var user = await userManager.FindByNameAsync(model.Email);
        if (user is null)
            // Don't reveal that the user does not exist
            return RedirectToAction(nameof(AccountController.ResetPasswordConfirmation), "Account");
        var result = await userManager.ResetPasswordAsync(user, model.Code, model.Password);
        if (result.Succeeded)
            return RedirectToAction(nameof(AccountController.ResetPasswordConfirmation), "Account");
        AddErrors(result);
        return View();
    }

    ///<summary>GET: /Account/ResetPasswordConfirmation</summary>
    [HttpGet]
    [AllowAnonymous]
    public IActionResult ResetPasswordConfirmation() => View();

    ///<summary>GET: /Account/SendCode</summary>
    [HttpGet]
    [AllowAnonymous]
    public async Task<ActionResult> SendCode(string returnUrl = default!, bool rememberMe = false)
    {
        returnUrl ??= string.Empty;
        var user = await signInManager.GetTwoFactorAuthenticationUserAsync();
        if (user is null)
            return View("Error");
        var userFactors = await userManager.GetValidTwoFactorProvidersAsync(user);
        var factorOptions = userFactors
            .Select(purpose => new SelectListItem { Text = purpose, Value = purpose })
            .ToList();
        return View(
            new SendCodeViewModel
            {
                Providers = factorOptions,
                ReturnUrl = returnUrl,
                RememberMe = rememberMe
            }
        );
    }

    ///<summary>POST: /Account/SendCode</summary>
    [HttpPost]
    [AllowAnonymous]
    [ValidateAntiForgeryToken]
    public async Task<IActionResult> SendCode(SendCodeViewModel model)
    {
        if (!ModelState.IsValid)
            return View();

        var user = await signInManager.GetTwoFactorAuthenticationUserAsync();
        if (user is null)
            return View("Error");

        // Generate the token and send it
        var code = await userManager.GenerateTwoFactorTokenAsync(user, model.SelectedProvider);
        if (string.IsNullOrWhiteSpace(code))
            return View("Error");

        var message = "Your security code is: " + code;
        // if (model.SelectedProvider == "Email")
        //     await _emailSender.SendEmailAsync(await userManager.GetEmailAsync(user), "Security Code", message);
        // else if (model.SelectedProvider == "Phone")
        //     await _smsSender.SendSmsAsync(await userManager.GetPhoneNumberAsync(user), message);

        return RedirectToAction(
            nameof(VerifyCode),
            new { Provider = model.SelectedProvider, model.ReturnUrl, model.RememberMe }
        );
    }

    ///<summary>GET: /Account/VerifyCode</summary>
    [HttpGet]
    [AllowAnonymous]
    public async Task<IActionResult> VerifyCode(string provider, bool rememberMe, string returnUrl = default!)
    {
        returnUrl ??= string.Empty;
        // Require that the user has already logged in via username/password or external login
        var user = await signInManager.GetTwoFactorAuthenticationUserAsync();
        return user is null
            ? View("Error")
            : View(
                new VerifyCodeViewModel
                {
                    Provider = provider,
                    ReturnUrl = returnUrl,
                    RememberMe = rememberMe
                }
            );
    }

    ///<summary>POST: /Account/VerifyCode</summary>
    [HttpPost]
    [AllowAnonymous]
    [ValidateAntiForgeryToken]
    public async Task<IActionResult> VerifyCode(VerifyCodeViewModel model)
    {
        if (!ModelState.IsValid)
            return View(model);

        // The following code protects for brute force attacks against the two factor codes.
        // If a user enters incorrect codes for a specified amount of time then the user account
        // will be locked out for a specified amount of time.
        var result = await signInManager.TwoFactorSignInAsync(
            model.Provider,
            model.Code,
            model.RememberMe,
            model.RememberBrowser
        );
        if (result.Succeeded)
            return RedirectToLocal(model.ReturnUrl);
        if (result.IsLockedOut)
            return View("Lockout");
        else
        {
            ModelState.AddModelError("", "Invalid code.");
            return View(model);
        }
    }

    #region Helpers

    private void AddErrors(IdentityResult result)
    {
        foreach (var error in result.Errors)
            ModelState.AddModelError(string.Empty, error.Description);
    }

    private async Task<FunicularUser> GetCurrentUserAsync() => await userManager.GetUserAsync(User);

    private IActionResult RedirectToLocal(string returnUrl) =>
        Url.IsLocalUrl(returnUrl) ? Redirect(returnUrl) : RedirectToAction(nameof(HomeController.Index), "Home");

    #endregion
}