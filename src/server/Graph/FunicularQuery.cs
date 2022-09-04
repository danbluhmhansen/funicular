namespace Funicular.Server.Graph;

using System.Linq.Expressions;

using Funicular.Server.Data;
using Funicular.Server.Data.Models;
using Funicular.Server.Graph.Models;

using GraphQL;
using GraphQL.Builders;
using GraphQL.MicrosoftDI;
using GraphQL.Types;

using Microsoft.EntityFrameworkCore;

internal class FunicularQuery : ObjectGraphType<object>
{
    private FieldBuilder<object, List<object>> charactersFieldBuilder;
    private readonly List<DynamicField> dynamicFields = new();

    public FunicularQuery()
    {
        Name = "Query";

        charactersFieldBuilder = Field<ListGraphType<CharacterType>, List<object>>("characters")
            .Argument<BooleanGraphType>("count")
            .Argument<IntGraphType>("top")
            .Argument<IntGraphType>("skip")
            .Argument<StringGraphType>("id")
            .Argument<StringGraphType>("name")
            .Argument<ListGraphType<OrderByGraphType>>("orderby");
    }

    public void AddDynamicFields(params DynamicField[] fields) => dynamicFields.AddRange(fields);

    public void AddDynamicFields(IEnumerable<DynamicField> fields) => AddDynamicFields(fields.ToArray());

    public static IQueryable<Character> DynamicFieldPredicate(
        IResolveFieldContext<object> context,
        IQueryable<Character> query,
        DynamicField field
    )
    {
        var fieldName = field.Name;
        switch (field.Type)
        {
            case "int":
                var intArgument = context.GetArgument<int?>(fieldName.ToCamelCase());
                return intArgument.HasValue
                    ? query.Where(
                        character =>
                            character.Json.HasValue
                            && character.Json.Value.GetProperty(fieldName).GetInt32() == intArgument.Value
                    )
                    : query;
            case "string":
                var stringArgument = context.GetArgument<string?>(fieldName.ToCamelCase());
                return !string.IsNullOrWhiteSpace(stringArgument)
                    ? query.Where(
                        character =>
                            character.Json.HasValue
                            && character.Json.Value.GetProperty(fieldName).GetString() == stringArgument
                    )
                    : query;
            default:
                throw new NotSupportedException();
        }
    }

    public FieldBuilder<object, List<object>> DynamicFieldArgument(DynamicField field) =>
        charactersFieldBuilder = field.Type switch
        {
            "int" => charactersFieldBuilder.Argument<IntGraphType>(field.Name),
            "string" => charactersFieldBuilder.Argument<StringGraphType>(field.Name),
            _ => throw new NotSupportedException(),
        };

    public FieldBuilder<object, List<object>> InitializeCharacters()
    {
        foreach (var field in dynamicFields)
            DynamicFieldArgument(field);
        return charactersFieldBuilder = charactersFieldBuilder
            .Resolve()
            .WithService<FunicularDbContext>()
            .ResolveAsync(
                async (context, db) =>
                {
                    var query = db.Characters.AsQueryable();

                    var count = context.GetArgument<bool>("count");
                    var countTask = count ? query.CountAsync(context.CancellationToken) : default;

                    var id = context.GetArgument<string>("id");
                    if (!string.IsNullOrWhiteSpace(id))
                        query = query.Where(character => EF.Functions.Like(character.Id.ToString(), $"%{id}%"));

                    var name = context.GetArgument<string>("name");
                    if (!string.IsNullOrWhiteSpace(name))
                        query = query.Where(character => EF.Functions.Like(character.Name, $"%{name}%"));

                    foreach (var field in dynamicFields)
                        query = DynamicFieldPredicate(context, query, field);

                    if (context.HasArgument("orderby"))
                        foreach ((var field, var desc) in context.GetArgument<IEnumerable<OrderBy>>("orderby"))
                        {
                            var pascalField = field.ToPascalCase();
                            var dynamicField = dynamicFields.FirstOrDefault(
                                dynamicField => dynamicField.Name == pascalField
                            );
                            Expression<Func<Character, object?>> keySelector = pascalField switch
                            {
                                nameof(Character.Id) => character => character.Id,
                                nameof(Character.Name) => character => character.Name,
                                _
                                    => dynamicField!.Type switch
                                    {
                                        "int"
                                            => character =>
                                                character.Json.HasValue
                                                    ? character.Json.Value.GetProperty(field).GetInt32()
                                                    : 0,
                                        "string"
                                            => character =>
                                                character.Json.HasValue
                                                    ? character.Json.Value.GetProperty(field).GetString()
                                                    : null,
                                        _ => throw new NotSupportedException(),
                                    },
                            };

                            query = query is IOrderedQueryable<Character> ordered
                                ? desc
                                    ? ordered.ThenByDescending(keySelector)
                                    : ordered.ThenBy(keySelector)
                                : desc
                                    ? query.OrderByDescending(keySelector)
                                    : query.OrderBy(keySelector);
                        }

                    var top = context.GetArgument<int>("top");
                    if (top is not 0)
                        query = query.Take(top);

                    var skip = context.GetArgument<int>("skip");
                    if (skip is not 0)
                        query = query.Skip(skip);

                    var selectId = context.SubFields?.ContainsKey("id") == true;
                    var selectName = context.SubFields?.ContainsKey("name") == true;

                    if (countTask is not null)
                        context.OutputExtensions["count"] = await countTask;

                    return await query
                        .Select(
                            character =>
                                new Character(
                                    selectId ? character.Id : default,
                                    selectName ? character.Name : string.Empty,
                                    character.Json
                                )
                        )
                        .OfType<object>()
                        .ToListAsync(context.CancellationToken);
                }
            );
    }
}