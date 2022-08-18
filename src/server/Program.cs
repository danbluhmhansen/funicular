using GraphQL;
using GraphQL.Types;

var builder = WebApplication.CreateBuilder(args);

// Add services to the container.
var services = builder.Services;

services.AddSingleton<CharacterType>();
services.AddSingleton<ApplicationQuery>();
services.AddGraphQL(options => options
    .AddSystemTextJson()
    .AddSchema<ApplicationSchema>());

var app = builder.Build();

app.UseGraphQL<ISchema>();

// Configure the HTTP request pipeline.
if (app.Environment.IsDevelopment())
{
    app.UseGraphQLPlayground();
}

app.UseHttpsRedirection();

app.Run();

public record Character(Guid Id, string Name, int Strength);

public class CharacterType : ObjectGraphType<Character>
{
    public CharacterType()
    {
        Name = "Character";
        Field(_ => _.Id);
        Field(_ => _.Name);
        Field(_ => _.Strength);
    }
}

public class ApplicationQuery : ObjectGraphType<object>
{
    public ApplicationQuery()
    {
        Name = "Query";
        Field<ListGraphType<CharacterType>>("characters")
            .Resolve(context => new Character[]
            {
                new(Guid.NewGuid(), "Foo", 15),
            });
    }
}

public class ApplicationSchema : Schema
{
    public ApplicationSchema(IServiceProvider services) : base(services)
    {
        Query = services.GetRequiredService<ApplicationQuery>();
    }
}

