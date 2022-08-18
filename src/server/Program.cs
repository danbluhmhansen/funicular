using Funicular.Server.Graph;
using Funicular.Server.Graph.Models;

using GraphQL;
using GraphQL.Types;

var builder = WebApplication.CreateBuilder(args);

// Add services to the container.
var services = builder.Services;

services.AddSingleton<CharacterType>();
services.AddSingleton<FunicularQuery>();
services.AddGraphQL(options => options
    .AddSystemTextJson()
    .AddSchema<FunicularSchema>());

var app = builder.Build();

app.UseGraphQL<ISchema>();

// Configure the HTTP request pipeline.
if (app.Environment.IsDevelopment())
{
    app.UseGraphQLPlayground();
}

app.UseHttpsRedirection();

app.Run();

