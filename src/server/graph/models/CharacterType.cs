namespace Funicular.Server.Graph.Models;

using GraphQL.Types;

internal class CharacterType : ObjectGraphType<object>
{
    public CharacterType()
    {
        Name = "Character";
        Field<IdGraphType>("id");
        Field<StringGraphType>("name");
        Field<IntGraphType>("strength");
        Field<IntGraphType>("dexterity");
        Field<IntGraphType>("constitution");
        Field<IntGraphType>("intelligence");
        Field<IntGraphType>("wisdom");
        Field<IntGraphType>("charisma");
    }
}

