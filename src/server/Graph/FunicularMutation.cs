namespace Funicular.Server.Graph;

using System.Text.Json.Nodes;

using Funicular.Server.Data;
using Funicular.Server.Data.Models;
using Funicular.Server.Graph.Models;

using GraphQL;
using GraphQL.Builders;
using GraphQL.MicrosoftDI;
using GraphQL.Types;

internal class FunicularMutation : ObjectGraphType
{
    public FunicularMutation()
    {
        Name = "Mutation";

        saveCharactersFieldBuilder = Field<CharacterType>("saveCharacter")
            .Argument<IdGraphType>("id")
            .Argument<StringGraphType>("name");

        Field<CharacterType>("dropCharacter")
            .Argument<NonNullGraphType<IdGraphType>>("id")
            .Resolve()
            .WithService<FunicularDbContext>()
            .ResolveAsync(
                async (context, db) =>
                {
                    var id = context.GetArgument<Guid>("id");
                    var character = await db.Characters.FindAsync(new object[] { id }, context.CancellationToken);
                    if (character is not null)
                    {
                        db.Characters.Remove(character);
                        return character;
                    }
                    return null;
                }
            );
    }

    private readonly List<DynamicField> dynamicFields = new();
    private FieldBuilder<object?, object> saveCharactersFieldBuilder;

    public void AddDynamicFields(params DynamicField[] fields) => dynamicFields.AddRange(fields);

    public void AddDynamicFields(IEnumerable<DynamicField> fields) => AddDynamicFields(fields.ToArray());

    public FieldBuilder<object?, object> DynamicFieldArgument(DynamicField field) =>
        saveCharactersFieldBuilder = field.Type switch
        {
            "int" => saveCharactersFieldBuilder.Argument<IntGraphType>(field.Name),
            "string" => saveCharactersFieldBuilder.Argument<StringGraphType>(field.Name),
            _ => throw new NotSupportedException(),
        };

    public FieldBuilder<object?, object> InitializeSaveCharacters()
    {
        foreach (var field in dynamicFields)
            DynamicFieldArgument(field);
        return saveCharactersFieldBuilder
            .Resolve()
            .WithService<FunicularDbContext>()
            .ResolveAsync(
                async (context, db) =>
                {
                    var id = context.GetArgument<Guid?>("id") ?? Guid.Empty;

                    var existing =
                        id != Guid.Empty ? await db.Characters.FindAsync(id, context.CancellationToken) : default;
                    var character = existing ?? new(id, string.Empty, default);

                    if (context.HasArgument("name"))
                        character = character with { Name = context.GetArgument<string>("name") };

                    db.Characters.Update(character);
                    return character;
                }
            );
    }
}