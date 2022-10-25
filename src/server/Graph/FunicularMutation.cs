namespace Funicular.Server.Graph;

using Funicular.Server.Attributes;
using Funicular.Server.Data;
using Funicular.Server.Data.Models;

using HotChocolate.Data.Filters.Expressions;
using HotChocolate.Resolvers;

using Microsoft.EntityFrameworkCore;

public class FunicularMutation
{
    public async Task<WeatherForecast> SaveWeatherForecastAsync(
        DateTime date,
        [DefaultValue(0)] Optional<int> temperatureC,
        [DefaultValue("")] Optional<string> summary,
        FunicularDbContext db,
        CancellationToken cancellationToken = default
    )
    {
        var existing = await db.WeatherForecasts.FindAsync(new object[] { date }, cancellationToken);
        var entity = existing ?? new(date, 0, string.Empty);

        if (temperatureC.HasValue)
            entity = entity with { TemperatureC = temperatureC };
        if (summary.HasValue)
            entity = entity with { Summary = summary };

        if (existing is null)
            db.WeatherForecasts.Add(entity);

        return entity;
    }

    [UseFiltering]
    public async Task<IEnumerable<WeatherForecast>> SetWeatherForecastsAsync(
        [DefaultDateTimeValue] Optional<DateTime> date,
        [DefaultValue(0)] Optional<int> temperatureC,
        [DefaultValue("")] Optional<string> summary,
        IResolverContext context,
        FunicularDbContext db,
        CancellationToken cancellationToken = default
    )
    {
        IEnumerable<WeatherForecast> entities = await db.WeatherForecasts
            .Filter(context)
            .ToListAsync(cancellationToken);
        entities = entities.Select(entity =>
        {
            if (date.HasValue)
                entity = entity with { Date = date };
            if (temperatureC.HasValue)
                entity = entity with { TemperatureC = temperatureC };
            if (summary.HasValue)
                entity = entity with { Summary = summary };
            return entity;
        });
        return entities;
    }

    [UseFiltering]
    public async Task<IEnumerable<WeatherForecast>> DropWeatherForecastsAsync(
        IResolverContext context,
        FunicularDbContext db,
        CancellationToken cancellationToken = default
    )
    {
        var entities = await db.WeatherForecasts.Filter(context).ToListAsync(cancellationToken);
        db.WeatherForecasts.RemoveRange(entities);
        return entities;
    }
}