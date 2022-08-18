namespace Funicular.Server.Graph;

using Funicular.Server.Data.Models;
using Funicular.Server.Graph.Models;

using GraphQL.Types;

public class FunicularQuery : ObjectGraphType<object>
{
    public FunicularQuery()
    {
        Name = "Query";
        Field<ListGraphType<CharacterType>>("characters")
            .Resolve(context => new Character[]
            {
                new(Guid.NewGuid(), "Foo", 15),
            });
    }
}

