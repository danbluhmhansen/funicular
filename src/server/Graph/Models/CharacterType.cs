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
                    .Resolve(
                        context =>
                            context.Source.Json.TryGetProperty(field.Name, out var value) ? value.GetInt32() : null
                    ),
            "string"
                => Field<StringGraphType>(field.Name)
                    .Resolve(
                        context =>
                            context.Source.Json.TryGetProperty(field.Name, out var value) ? value.GetString() : null
                    ),
            _ => throw new NotSupportedException(),
        };
}