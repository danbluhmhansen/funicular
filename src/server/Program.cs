using Funicular.Server.Data;
using Funicular.Server.Graph;
using Funicular.Server.Graph.Models;
#if DEBUG
using Funicular.Server.Services;
#endif

using GraphQL;
using GraphQL.Types;

using Microsoft.EntityFrameworkCore;

var builder = WebApplication.CreateBuilder(args);

// Add services to the container.
var services = builder.Services;

services.AddDbContext<FunicularDbContext>(options => options.UseNpgsql(builder.Configuration.GetConnectionString("Default")));

services.AddSingleton<CharacterType>();
services.AddSingleton<FunicularQuery>();
services.AddGraphQL(options => options
    .AddSystemTextJson()
    .AddSchema<FunicularSchema>());

#if DEBUG
services.AddHostedService<DataSeedWorker>();
#endif

var app = builder.Build();

app.UseGraphQL<ISchema>();

// Configure the HTTP request pipeline.
if (app.Environment.IsDevelopment())
{
    app.UseGraphQLPlayground();
}

app.UseHttpsRedirection();

app.Run();

