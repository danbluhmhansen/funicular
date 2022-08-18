#if DEBUG
namespace Funicular.Server.Services;

using System.Text.Json;
using System.Threading;
using System.Threading.Tasks;

using Bogus;

using Funicular.Server.Data;
using Funicular.Server.Data.Models;

using Microsoft.EntityFrameworkCore;

using MoreLinq;

internal class DataSeedWorker : IHostedService
{
    public DataSeedWorker(IServiceProvider services)
    {
        this.services = services;
    }

    private readonly IServiceProvider services;

    public async Task StartAsync(CancellationToken cancellationToken)
    {
        await using var scope = this.services.CreateAsyncScope();
        var services = scope.ServiceProvider;

        var db = services.GetRequiredService<FunicularDbContext>();

        if (!await db.Characters.AnyAsync(cancellationToken))
        {
            var standardArray = new[] { 15, 14, 13, 12, 10, 8 };
            db.Characters.AddRange(new Faker<Character>()
                .CustomInstantiator(f =>
                {
                    var stats = standardArray.Shuffle().ToArray();
                    return new(
                        Guid.NewGuid(),
                        f.Name.FirstName(),
                        JsonSerializer.SerializeToElement(
                            new Dictionary<string, object?>
                            {
                                { "Strength", stats[0] },
                                { "Dexterity", stats[1] },
                                { "Constitution", stats[2] },
                                { "Intelligence", stats[3] },
                                { "Wisdom", stats[4] },
                                { "Charisma", stats[5] },
                            }));
                })
                .Generate(100));
        }

        await db.SaveChangesAsync(cancellationToken);
    }

    public Task StopAsync(CancellationToken cancellationToken) => Task.CompletedTask;
}
#endif
