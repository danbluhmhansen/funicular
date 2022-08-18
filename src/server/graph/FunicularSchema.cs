namespace Funicular.Server.Graph;

using GraphQL.Types;

public class FunicularSchema : Schema
{
    public FunicularSchema(IServiceProvider services) : base(services)
    {
        Query = services.GetRequiredService<FunicularQuery>();
    }
}

