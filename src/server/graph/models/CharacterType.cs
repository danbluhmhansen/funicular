namespace Funicular.Server.Graph.Models;

using Funicular.Server.Data.Models;

using GraphQL.Types;

internal class CharacterType : ObjectGraphType<Character>
{
    public CharacterType()
    {
        Name = "Character";
        Field(_ => _.Id);
        Field(_ => _.Name);
    }
}

