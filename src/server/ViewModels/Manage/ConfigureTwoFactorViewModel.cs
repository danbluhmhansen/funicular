namespace Funicular.Server.ViewModels.Manage;

using Microsoft.AspNetCore.Mvc.Rendering;

public class ConfigureTwoFactorViewModel
{
    public string SelectedProvider { get; set; }

    public ICollection<SelectListItem> Providers { get; set; }
}