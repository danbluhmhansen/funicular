namespace Funicular.Client.Pages;

using System.Collections.Generic;

using Funicular.Client.Graph;

using Microsoft.AspNetCore.Components;

public partial class FetchData : ComponentBase
{
    private IReadOnlyList<IGetWeatherForecasts_WeatherForecasts_Nodes>? forecasts;

    [Inject]
    private FunicularClient Client { get; set; }

    protected override async Task OnInitializedAsync()
    {
        var result = await Client.GetWeatherForecasts.ExecuteAsync(10);
        forecasts = result.Data?.WeatherForecasts?.Nodes;
    }
}
