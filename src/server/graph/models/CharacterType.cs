namespace Funicular.Server.Graph.Models;

using Funicular.Server.Data.Models;

using GraphQL.Types;

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

