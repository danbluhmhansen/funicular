namespace Funicular.Server.Graph.Models;

using Funicular.Server.Data.Models;

using GraphQL.Builders;
using GraphQL.Types;

internal class CharacterType : ObjectGraphType<object>
{
    public CharacterType()
    {
        Name = "Character";
        Field<IdGraphType>("id");
        Field<StringGraphType>("name");
    }

    public FieldBuilder<object, object> CharacterField(CharacterField field) => field.Type switch
    {
        "int" => Field<IntGraphType>(field.Name),
        "string" => Field<StringGraphType>(field.Name),
        _ => throw new NotSupportedException(),
    };
}

