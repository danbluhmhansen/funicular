namespace Funicular.Server.Graph;

using Funicular.Server.Data;
using Funicular.Shared;

public class FunicularQuery
{
    [UsePaging(DefaultPageSize = 100, MaxPageSize = 100), UseFiltering, UseSorting]
    public IQueryable<WeatherForecast> GetWeatherForecasts(FunicularDbContext db) => db.WeatherForecasts;
}