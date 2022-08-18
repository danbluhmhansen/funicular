namespace Funicular.Server.Graph;

using GraphQL.Types;

internal class FunicularSchema : Schema
{
    public FunicularSchema(IServiceProvider services) : base(services)
    {
        Query = services.GetRequiredService<FunicularQuery>();
    }
}

