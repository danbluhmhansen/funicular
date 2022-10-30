namespace Funicular.Server.Graph;

using Funicular.Server.Attributes;
using Funicular.Server.Commands;
using Funicular.Server.Data.Models;

using HotChocolate.Data.Filters.Expressions;
using HotChocolate.Resolvers;

public class FunicularMutation
{
    public async Task<WeatherForecast> SaveWeatherForecastAsync(
        DateTime date,
        [DefaultValue(0)] Optional<int> temperatureC,
        [DefaultValue("")] Optional<string> summary,
        IExecutable<WeatherForecast> executable,
        AddEntity addEntity,
        CancellationToken cancellationToken = default
    )
    {
        if (executable is not QueryableExecutable<WeatherForecast> query)
            throw new NotSupportedException();

        var existing =
            await query.WithSource(query.Source.Where(_ => _.Date == date)).FirstOrDefaultAsync(cancellationToken)
            as WeatherForecast;
        var entity = existing ?? new(date, 0, string.Empty);

        if (temperatureC.HasValue)
            entity = entity with { TemperatureC = temperatureC };
        if (summary.HasValue)
            entity = entity with { Summary = summary };

        if (existing is null)
            addEntity.Add(entity);

        return entity;
    }

    [UseFiltering]
    public async Task<IEnumerable<WeatherForecast>> SetWeatherForecastsAsync(
        [DefaultDateTimeValue] Optional<DateTime> date,
        [DefaultValue(0)] Optional<int> temperatureC,
        [DefaultValue("")] Optional<string> summary,
        IResolverContext context,
        IExecutable<WeatherForecast> executable,
        CancellationToken cancellationToken = default
    )
    {
        if (executable is not QueryableExecutable<WeatherForecast> query)
            throw new NotSupportedException();

        var entities =
            await query.Filter(context).ToListAsync(cancellationToken) as IEnumerable<WeatherForecast>
            ?? Enumerable.Empty<WeatherForecast>();
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
        IExecutable<WeatherForecast> executable,
        RemoveEntity removeEntity,
        CancellationToken cancellationToken = default
    )
    {
        if (executable is not QueryableExecutable<WeatherForecast> query)
            throw new NotSupportedException();

        var entities =
            await query.Filter(context).ToListAsync(cancellationToken) as IEnumerable<WeatherForecast>
            ?? Enumerable.Empty<WeatherForecast>();
        removeEntity.RemoveRange(entities);
        return entities;
    }
}