namespace Funicular.Server.Graph.Models;

using GraphQL.Types;

internal class OrderByGraphType : InputObjectGraphType<OrderBy>
{
    public OrderByGraphType()
    {
        Name = "orderby";
        Field(orderby => orderby.Field);
        Field(orderby => orderby.Desc, true);
    }
}

internal record OrderBy(string Field, bool Desc = false);