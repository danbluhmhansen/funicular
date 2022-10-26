namespace Funicular.Server.Graph;

using Funicular.Server.Data.Models;

public class FunicularQuery
{
    [UsePaging(DefaultPageSize = 100, MaxPageSize = 100), UseFiltering, UseSorting]
    public IExecutable<WeatherForecast> GetWeatherForecasts(IExecutable<WeatherForecast> executable) => executable;
}