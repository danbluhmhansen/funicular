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

services.AddDbContext<FunicularDbContext>(
    options => options.UseNpgsql(builder.Configuration.GetConnectionString("Default"))
);

services.AddScoped<OrderByGraphType>();
services.AddScoped<CharacterType>();
services.AddScoped<FunicularQuery>();
services.AddScoped<FunicularMutation>();

services.AddGraphQL(
    options => options.AddSystemTextJson().AddSchema<FunicularSchema>(GraphQL.DI.ServiceLifetime.Scoped)
);

services.AddControllers();

if (builder.Environment.IsDevelopment())
    services.AddHostedService<DataSeedWorker>();

var app = builder.Build();

app.Use(
    async (context, next) =>
    {
        var characterType = context.RequestServices.GetRequiredService<CharacterType>();
        var query = context.RequestServices.GetRequiredService<FunicularQuery>();
        var mutation = context.RequestServices.GetRequiredService<FunicularMutation>();
        var db = context.RequestServices.GetRequiredService<FunicularDbContext>();
        await foreach (var field in db.CharacterFields.AsAsyncEnumerable().WithCancellation(context.RequestAborted))
        {
            characterType.DynamicField(field);
            query.AddDynamicFields(field);
        }
        query.InitializeCharacters();
        mutation.InitializeSaveCharacters();
        await next();
    }
);

// Configure the HTTP request pipeline.
if (app.Environment.IsDevelopment())
{
    app.UseGraphQLPlayground();
}

app.UseHttpsRedirection();

app.UseRouting();

app.MapControllers();

app.Run();