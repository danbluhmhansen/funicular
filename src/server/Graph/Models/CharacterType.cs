namespace Funicular.Server.Graph.Models;

using Funicular.Server.Data.Models;

using GraphQL.Builders;
using GraphQL.Types;

internal class CharacterType : ObjectGraphType<Character>
{
    public CharacterType()
    {
        Name = "Character";
        Field<IdGraphType>("id").Resolve(context => context.Source.Id);
        Field<StringGraphType>("name").Resolve(context => context.Source.Name);
    }

    public FieldBuilder<Character, object> DynamicField(DynamicField field) =>
        field.Type switch
        {
            "int"
                => Field<IntGraphType>(field.Name)
                    .Resolve(context => context.Source.Json.GetProperty(field.Name).GetInt32()),
            "string"
                => Field<StringGraphType>(field.Name)
                    .Resolve(context => context.Source.Json.GetProperty(field.Name).GetString()),
            _ => throw new NotSupportedException(),
        };
}