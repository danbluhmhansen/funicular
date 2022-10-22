namespace Funicular.Server.Models.Account;

using System.ComponentModel.DataAnnotations;

public record Login(string Email, string Password, [property: Display(Name = "Remember me?")] bool RememberMe = false);