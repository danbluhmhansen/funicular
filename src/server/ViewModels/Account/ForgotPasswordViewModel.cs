using System.ComponentModel.DataAnnotations;

namespace Funicular.Server.ViewModels.Account;

public class ForgotPasswordViewModel
{
    [Required]
    [EmailAddress]
    public string Email { get; set; }
}