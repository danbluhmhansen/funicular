using Funicular.Server.Data;
using Funicular.Server.Graph;
using Funicular.Server.Graph.Models;
using Funicular.Server.Services;

using GraphQL;
using GraphQL.Types;

using Microsoft.EntityFrameworkCore;

var builder = WebApplication.CreateBuilder(args);

// Add services to the container.
var services = builder.Services;

services.AddDbContext<FunicularDbContext>(options => options.UseNpgsql(builder.Configuration.GetConnectionString("Default")));

services.AddScoped<CharacterType>();
services.AddScoped<FunicularQuery>();
services.AddGraphQL(options => options
    .AddSystemTextJson()
    .AddSchema<FunicularSchema>(GraphQL.DI.ServiceLifetime.Scoped));

if (builder.Environment.IsDevelopment())
    services.AddHostedService<DataSeedWorker>();

var app = builder.Build();

app.Use(async (context, next) =>
{
    var characterType = context.RequestServices.GetRequiredService<CharacterType>();
    var query = context.RequestServices.GetRequiredService<FunicularQuery>();
    var db = context.RequestServices.GetRequiredService<FunicularDbContext>();
    await foreach (var field in db.CharacterFields.AsAsyncEnumerable().WithCancellation(context.RequestAborted))
    {
        characterType.CharacterField(field);
        query.AddCharacterFields(field);
    }
    query.InitializeCharacters();
    await next.Invoke();
});
app.UseGraphQL<ISchema>();

// Configure the HTTP request pipeline.
if (app.Environment.IsDevelopment())
{
    app.UseGraphQLPlayground();
}

app.UseHttpsRedirection();

app.Run();

