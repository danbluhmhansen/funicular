namespace Funicular.Server.Services;

using System.Threading;
using System.Threading.Tasks;

using Funicular.Server.Data;
using Funicular.Server.Data.Models;

using Microsoft.EntityFrameworkCore;

public class DataSeedWorker : IHostedService
{
    public DataSeedWorker(IServiceProvider serviceProvider)
    {
        this.serviceProvider = serviceProvider;
    }

    private static readonly string[] Summaries = new[]
    {
        "Freezing",
        "Bracing",
        "Chilly",
        "Cool",
        "Mild",
        "Warm",
        "Balmy",
        "Hot",
        "Sweltering",
        "Scorching"
    };
    private readonly IServiceProvider serviceProvider;

    public async Task StartAsync(CancellationToken cancellationToken)
    {
        await using var scope = serviceProvider.CreateAsyncScope();
        var services = scope.ServiceProvider;
        var db = services.GetRequiredService<FunicularDbContext>();

        if (!await db.WeatherForecasts.AnyAsync(cancellationToken))
        {
            var today = DateTime.Today;
            db.WeatherForecasts.AddRange(
                Enumerable
                    .Range(1, 100)
                    .Select(
                        i =>
                            new WeatherForecast(
                                today.AddDays(-i),
                                Random.Shared.Next(-20, 55),
                                Summaries[Random.Shared.Next(Summaries.Length)]
                            )
                    )
            );
            await db.SaveChangesAsync(cancellationToken);
        }
    }

    public Task StopAsync(CancellationToken cancellationToken) => Task.CompletedTask;
}