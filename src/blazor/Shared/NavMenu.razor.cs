namespace Funicular.Client.Shared;

public partial class NavMenu
{
    private bool active = false;

    private void ToggleNavMenu() => active = !active;
}
