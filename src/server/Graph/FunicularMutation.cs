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
            .Argument<StringGraphType>("name")
            .Resolve()
            .WithScope()
            .WithService<FunicularDbContext>()
            .ResolveAsync(
                async (context, db) =>
                {
                    var id = context.GetArgument<Guid?>("id") ?? Guid.Empty;
                    var name = context.GetArgument<string>("name");
                    Character character;
                    if (id != Guid.Empty)
                    {
                        var existing = await db.Characters.FindAsync(id, context.CancellationToken);
                        if (existing is not null)
                        {
                            if (name is not null)
                                existing = existing with { Name = name };
                            character = existing;
                        }
                        else
                        {
                            character = new(id, name, default);
                        }
                    }
                    else
                    {
                        character = new(Guid.Empty, name, default);
                    }
                    db.Characters.Update(character);
                    return character;
                }
            );
    }
}