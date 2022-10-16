namespace Funicular.Server.Graph;

using System.Text.Json;
using System.Text.Json.Nodes;

using Funicular.Server.Data;
using Funicular.Server.Data.Models;
using Funicular.Server.Graph.Models;

using GraphQL;
using GraphQL.Builders;
using GraphQL.MicrosoftDI;
using GraphQL.Types;

using Microsoft.EntityFrameworkCore;

internal class FunicularMutation : ObjectGraphType
{
    public FunicularMutation()
    {
        Name = "Mutation";

        base.Field<DynamicFieldType>("saveCharacterField")
            .Argument<NonNullGraphType<StringGraphType>>("name")
            .Argument<StringGraphType>("type")
            .Argument<BooleanGraphType>("required")
            .Resolve()
            .WithService<FunicularDbContext>()
            .ResolveAsync(
                async (context, db) =>
                {
                    var name = context.GetArgument<string>("name");
                    var update = await db.CharacterFields.AnyAsync(e => e.Name == name);
                    DynamicField entity =
                        new(
                            name,
                            context.HasArgument("type") ? context.GetArgument<string>("type") : "string",
                            context.HasArgument("required") && context.GetArgument<bool>("required")
                        );
                    if (update)
                        db.CharacterFields.Update(entity);
                    else
                        db.CharacterFields.Add(entity);
                    return entity;
                }
            );

        Field<DynamicFieldType>("dropCharacterField")
            .Argument<NonNullGraphType<StringGraphType>>("name")
            .Resolve()
            .WithService<FunicularDbContext>()
            .ResolveAsync(
                async (context, db) =>
                {
                    var name = context.GetArgument<string>("name");
                    var entity = await db.CharacterFields.FindAsync(new object[] { name }, context.CancellationToken);
                    if (entity is not null)
                    {
                        db.CharacterFields.Remove(entity);
                        return entity;
                    }
                    return default;
                }
            );

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
                    return default;
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

    public static JsonNode? GetDynamicField(IResolveFieldContext<object?> context, DynamicField field) =>
        field.Type switch
        {
            "int" => context.GetArgument<int>(field.Name),
            "string" => context.GetArgument<string>(field.Name),
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
                        id != Guid.Empty
                            ? await db.Characters
                                .AsNoTracking()
                                .FirstOrDefaultAsync(character => character.Id == id, context.CancellationToken)
                            : default;
                    var character =
                        existing
                        ?? new(id, string.Empty, JsonSerializer.SerializeToElement(new Dictionary<string, object?>()));

                    if (context.HasArgument("name"))
                        character = character with { Name = context.GetArgument<string>("name") };

                    var dynamicFields = this.dynamicFields.Where(field => context.HasArgument(field.Name));
                    if (dynamicFields.Any())
                    {
                        var json = JsonObject.Create(character.Json) ?? new();
                        foreach (var field in dynamicFields)
                            json[field.Name] = GetDynamicField(context, field);
                        character = character with { Json = JsonDocument.Parse(json.ToJsonString()).RootElement };
                    }

                    db.Characters.Update(character);
                    return character;
                }
            );
    }
}