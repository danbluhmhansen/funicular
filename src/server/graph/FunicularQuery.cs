namespace Funicular.Server.Graph;

using Funicular.Server.Data;
using Funicular.Server.Data.Models;
using Funicular.Server.Graph.Models;

using GraphQL.MicrosoftDI;
using GraphQL.Types;

using Microsoft.EntityFrameworkCore;

internal class FunicularQuery : ObjectGraphType<object>
{
    public FunicularQuery()
    {
        Name = "Query";

        Field<ListGraphType<CharacterType>, List<Character>>("characters")
            .Resolve()
            .WithScope()
            .WithService<FunicularDbContext>()
            .ResolveAsync((context, db) =>
            {
                var selectId = context.SubFields?.ContainsKey("id") == true;
                var selectName = context.SubFields?.ContainsKey("name") == true;
                return db.Characters
                    .Select(character => new Character(
                        selectId ? character.Id : default,
                        selectName ? character.Name : string.Empty,
                        default))
                    .ToListAsync(context.CancellationToken) as Task<List<Character>?>;
            });
    }
}

