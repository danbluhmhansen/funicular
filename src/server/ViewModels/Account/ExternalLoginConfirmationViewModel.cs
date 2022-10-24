using System.ComponentModel.DataAnnotations;

namespace Funicular.Server.ViewModels.Account;

public class ExternalLoginConfirmationViewModel
{
    [Required]
    [EmailAddress]
    public string Email { get; set; }
}