namespace Funicular.Server.Services;

using System.Threading;
using System.Threading.Tasks;

using Funicular.Server.Data;
using Funicular.Server.Data.Models;

using Microsoft.EntityFrameworkCore;

using MoreLinq;

using OpenIddict.Abstractions;

using static OpenIddict.Abstractions.OpenIddictConstants;

public class DataSeedWorker : IHostedService
{
    public DataSeedWorker(IServiceProvider serviceProvider)
    {
        this.serviceProvider = serviceProvider;
    }

    private static readonly string[] Names = new[]
    {
        "Nhedar Duhrar",
        "Medem Shuhlem",
        "Bekrork Keentail",
        "Ragrodd Heartjumper",
        "Gligal Bask",
        "Fan Burgosk",
        "Adrem Wolfthorne",
        "Stran Mirthbrand",
        "Thahviohko Jovruthrib",
        "Sok-Vof Hezdahk",
        "Mergondar Gruldavyede",
        "Gorvad Gronama",
        "Hey Wiaon",
        "Fen Ying",
        "Varfumi Fabrindin",
        "Pirvar Sanzolbil",
    };

    private static readonly IEnumerable<string> AbilityNames = new[]
    {
        "strength",
        "dexterity",
        "constitution",
        "intelligence",
        "wisdom",
        "charisma",
    };
    private static readonly IEnumerable<int> StandardArray = new[] { 8, 10, 12, 13, 14, 15, };

    private static IDictionary<string, int> RandomAbilityScores =>
        AbilityNames.Zip(StandardArray.Shuffle()).ToDictionary();

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

        var applicationManager = services.GetRequiredService<IOpenIddictApplicationManager>();
        if (await applicationManager.CountAsync(cancellationToken) < 1)
            await applicationManager.CreateAsync(
                new OpenIddictApplicationDescriptor
                {
                    ClientId = "default",
                    ConsentType = ConsentTypes.Explicit,
                    DisplayName = "Funicular",
                    Permissions =
                    {
                        Permissions.Endpoints.Authorization,
                        Permissions.Endpoints.Logout,
                        Permissions.Endpoints.Token,
                        Permissions.GrantTypes.AuthorizationCode,
                        Permissions.GrantTypes.RefreshToken,
                        Permissions.ResponseTypes.Code,
                        Permissions.Scopes.Email,
                        Permissions.Scopes.Profile,
                        Permissions.Scopes.Roles,
                    },
                    PostLogoutRedirectUris = { new("https://localhost:7000/authentication/logout-callback"), },
                    RedirectUris = { new("https://localhost:7000/authentication/login-callback"), },
                    Requirements = { Requirements.Features.ProofKeyForCodeExchange, },
                    Type = ClientTypes.Public,
                },
                cancellationToken
            );

        var db = services.GetRequiredService<FunicularDbContext>();

        if (!await db.Characters.AnyAsync(cancellationToken))
            db.Characters.AddRange(Names.Select(name => new Character(CharacterId.New(), name, RandomAbilityScores)));

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
        }

        if (db.ChangeTracker.HasChanges())
            await db.SaveChangesAsync(cancellationToken);
    }

    public Task StopAsync(CancellationToken cancellationToken) => Task.CompletedTask;
}