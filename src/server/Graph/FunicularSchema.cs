namespace Funicular.Server.Graph;

using GraphQL.Instrumentation;
using GraphQL.Types;

internal class FunicularSchema : Schema
{
    public FunicularSchema(
        IServiceProvider services,
        FunicularQuery query,
        FunicularMutation mutation,
        IEnumerable<IFieldMiddleware> middlewares
    ) : base(services)
    {
        Query = query;
        Mutation = mutation;

        foreach (var middleware in middlewares)
            FieldMiddleware.Use(middleware);
    }
}