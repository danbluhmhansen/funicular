namespace Funicular.Server.Graph;

using Funicular.Server.Data;
using Funicular.Server.Data.Models;
using Funicular.Server.Graph.Models;

using GraphQL;
using GraphQL.MicrosoftDI;
using GraphQL.Types;

internal class FunicularMutation : ObjectGraphType
{
    public FunicularMutation()
    {
        Name = "Mutation";

        Field<CharacterType>("saveCharacter")
            .Argument<IdGraphType>("id")
            .Argument<NonNullGraphType<StringGraphType>>("name")
            .Resolve()
            .WithScope()
            .WithService<FunicularDbContext>()
            .ResolveAsync(
                async (context, db) =>
                {
                    var id = context.GetArgument<Guid?>("id");
                    Character character;
                    if (id.HasValue)
                    {
                        var existing = await db.Characters.FindAsync(id.Value, context.CancellationToken);
                        character = existing ?? new(id.Value, context.GetArgument<string>("name"), default);
                    }
                    else
                        character = new(Guid.Empty, context.GetArgument<string>("name"), default);
                    db.Characters.Update(character);
                    return character;
                }
            );
    }
}