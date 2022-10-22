namespace Funicular.Server.Models.Authorization;

using System.ComponentModel.DataAnnotations;

public record Authorize([property: Display(Name = "Application")] string ApplicationName, string Scope);