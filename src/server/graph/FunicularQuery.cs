namespace Funicular.Server.Graph;

using Funicular.Server.Data;
using Funicular.Server.Graph.Models;

using GraphQL;
using GraphQL.MicrosoftDI;
using GraphQL.Types;

using Microsoft.EntityFrameworkCore;

internal class FunicularQuery : ObjectGraphType<object>
{
    public FunicularQuery()
    {
        Name = "Query";

        Field<ListGraphType<CharacterType>, List<object>>("characters")
            .Argument<StringGraphType>("id")
            .Argument<StringGraphType>("name")
            .Argument<IntGraphType>("strength")
            .Argument<IntGraphType>("dexterity")
            .Argument<IntGraphType>("constitution")
            .Argument<IntGraphType>("intelligence")
            .Argument<IntGraphType>("wisdom")
            .Argument<IntGraphType>("charisma")
            .Resolve()
            .WithScope()
            .WithService<FunicularDbContext>()
            .ResolveAsync((context, db) =>
            {
                var query = db.Characters.AsQueryable();

                var idArgument = context.GetArgument<string>("id");
                if (!string.IsNullOrWhiteSpace(idArgument))
                    query = query.Where(character => EF.Functions.Like(character.Id.ToString(), $"%{idArgument}%"));

                var nameArgument = context.GetArgument<string>("name");
                if (!string.IsNullOrWhiteSpace(nameArgument))
                    query = query.Where(character => EF.Functions.Like(character.Name, $"%{nameArgument}%"));

                var strengthArgument = context.GetArgument<int?>("strength");
                if (strengthArgument.HasValue)
                    query = query.Where(character => character.Json.GetProperty("Strength").GetInt32() == strengthArgument.Value);

                var dexterityArgument = context.GetArgument<int?>("dexterity");
                if (dexterityArgument.HasValue)
                    query = query.Where(character => character.Json.GetProperty("Dexterity").GetInt32() == dexterityArgument.Value);

                var constitutionArgument = context.GetArgument<int?>("constitution");
                if (constitutionArgument.HasValue)
                    query = query.Where(character => character.Json.GetProperty("Constitution").GetInt32() == constitutionArgument.Value);

                var intelligenceArgument = context.GetArgument<int?>("intelligence");
                if (intelligenceArgument.HasValue)
                    query = query.Where(character => character.Json.GetProperty("Intelligence").GetInt32() == intelligenceArgument.Value);

                var wisdomArgument = context.GetArgument<int?>("wisdom");
                if (wisdomArgument.HasValue)
                    query = query.Where(character => character.Json.GetProperty("Wisdom").GetInt32() == wisdomArgument.Value);

                var charismaArgument = context.GetArgument<int?>("charisma");
                if (charismaArgument.HasValue)
                    query = query.Where(character => character.Json.GetProperty("Charisma").GetInt32() == charismaArgument.Value);

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
                        dexterity = selectStrength ? character.Json.GetProperty("Dexterity").GetInt32() : default,
                        constitution = selectStrength ? character.Json.GetProperty("Constitution").GetInt32() : default,
                        intelligence = selectStrength ? character.Json.GetProperty("Intelligence").GetInt32() : default,
                        wisdom = selectStrength ? character.Json.GetProperty("Wisdom").GetInt32() : default,
                        charisma = selectStrength ? character.Json.GetProperty("Charisma").GetInt32() : default,
                    })
                    .OfType<object>()
                    .ToListAsync(context.CancellationToken) as Task<List<object>?>;
            });
    }
}

