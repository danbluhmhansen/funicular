namespace Funicular.Server.Graph;

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
    private readonly List<CharacterField> characterFields = new();

    public FunicularQuery()
    {
        Name = "Query";

        charactersFieldBuilder = Field<ListGraphType<CharacterType>, List<object>>("characters")
            .Argument<StringGraphType>("id")
            .Argument<StringGraphType>("name");
    }

    public void AddCharacterFields(params CharacterField[] fields) => characterFields.AddRange(fields);
    public void AddCharacterFields(IEnumerable<CharacterField> fields) => AddCharacterFields(fields.ToArray());

    public static IQueryable<Character> CharacterFieldPredicate(IResolveFieldContext<object> context, IQueryable<Character> query, CharacterField field)
    {
        var fieldName = field.Name;
        switch (field.Type)
        {
            case "int":
                var intArgument = context.GetArgument<int?>(fieldName.ToCamelCase());
                return intArgument.HasValue
                    ? query.Where(character => character.Json.GetProperty(fieldName).GetInt32() == intArgument.Value)
                    : query;
            case "string":
                var stringArgument = context.GetArgument<string?>(fieldName.ToCamelCase());
                return !string.IsNullOrWhiteSpace(stringArgument)
                    ? query.Where(character => character.Json.GetProperty(fieldName).GetString() == stringArgument)
                    : query;
            default:
                throw new NotSupportedException();
        }
    }

    public FieldBuilder<object, List<object>> CharacterFieldArgument(CharacterField field) =>
        charactersFieldBuilder = field.Type switch
        {
            "int" => charactersFieldBuilder.Argument<IntGraphType>(field.Name),
            "string" => charactersFieldBuilder.Argument<StringGraphType>(field.Name),
            _ => throw new NotSupportedException(),
        };

    public FieldBuilder<object, List<object>> InitializeCharacters()
    {
        foreach (var field in characterFields)
            CharacterFieldArgument(field);
        return charactersFieldBuilder = charactersFieldBuilder.Resolve().WithScope().WithService<FunicularDbContext>()
            .ResolveAsync((context, db) =>
            {
                var query = db.Characters.AsQueryable();

                var idArgument = context.GetArgument<string>("id");
                if (!string.IsNullOrWhiteSpace(idArgument))
                    query = query.Where(character => EF.Functions.Like(character.Id.ToString(), $"%{idArgument}%"));

                var nameArgument = context.GetArgument<string>("name");
                if (!string.IsNullOrWhiteSpace(nameArgument))
                    query = query.Where(character => EF.Functions.Like(character.Name, $"%{nameArgument}%"));

                foreach (var field in characterFields)
                    query = CharacterFieldPredicate(context, query, field);

                var selectId = context.SubFields?.ContainsKey("id") == true;
                var selectName = context.SubFields?.ContainsKey("name") == true;
                var selectStrength = context.SubFields?.ContainsKey("strength") == true;
                var selectDexterity = context.SubFields?.ContainsKey("dexterity") == true;
                var selectConstitution = context.SubFields?.ContainsKey("constitution") == true;
                var selectIntelligence = context.SubFields?.ContainsKey("intelligence") == true;
                var selectWisdom = context.SubFields?.ContainsKey("wisdom") == true;
                var selectCharisma = context.SubFields?.ContainsKey("charisma") == true;
                return query
                    .Select(character => new
                    {
                        id = selectId ? character.Id.ToString() : default,
                        name = selectName ? character.Name : string.Empty,
                        strength = selectStrength ? character.Json.GetProperty("Strength").GetInt32() : default,
                        dexterity = selectDexterity ? character.Json.GetProperty("Dexterity").GetInt32() : default,
                        constitution = selectConstitution ? character.Json.GetProperty("Constitution").GetInt32() : default,
                        intelligence = selectIntelligence ? character.Json.GetProperty("Intelligence").GetInt32() : default,
                        wisdom = selectWisdom ? character.Json.GetProperty("Wisdom").GetInt32() : default,
                        charisma = selectCharisma ? character.Json.GetProperty("Charisma").GetInt32() : default,
                    })
                    .OfType<object>()
                    .ToListAsync(context.CancellationToken) as Task<List<object>?>;
            });
    }
}

