namespace Funicular.Server.Graph.Models;

using Funicular.Server.Data.Models;

using GraphQL.Types;

internal class DynamicFieldType : ObjectGraphType<DynamicField>
{
    public DynamicFieldType()
    {
        Name = "DynamicField";

        Field<StringGraphType>("name").Resolve(context => context.Source.Name);
        Field<StringGraphType>("type").Resolve(context => context.Source.Type);
        Field<BooleanGraphType>("required").Resolve(context => context.Source.Required);
    }
}