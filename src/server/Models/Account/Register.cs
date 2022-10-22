namespace Funicular.Server.Models.Account;

using System.ComponentModel.DataAnnotations;

public record Register(string Email, string Password, [property: Display(Name = "Password")] string ConfirmPassword);