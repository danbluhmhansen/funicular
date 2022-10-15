namespace Funicular.Client.Pages;

using System.Threading.Tasks;

using Funicular.Client.Models;

using GraphQL;
using GraphQL.Client.Abstractions;

using Microsoft.AspNetCore.Components;

public partial class Characters : ComponentBase
{
    [Parameter]
    [SupplyParameterFromQuery]
    public int PageIndex { get; set; }

    [Parameter]
    [SupplyParameterFromQuery]
    public int PageSize { get; set; }

    [Inject]
    public IGraphQLClient? GraphClient { get; set; }

    [Inject]
    public NavigationManager? Navigation { get; set; }

    private int? count;
    private int? pageCount;
    private IEnumerable<Character>? characters;

    private IEnumerable<int> Pages =>
        pageCount.HasValue ? Enumerable.Range(1, pageCount.Value + 1) : Enumerable.Empty<int>();

    private bool HasPreviousPage => PageIndex is not 0;
    private bool HasNextPage => PageIndex + 1 < pageCount;

    protected override async Task OnParametersSetAsync()
    {
        if (GraphClient is null)
            return;

        if (PageSize is 0)
            PageSize = 10;

        var response = await GraphClient.SendQueryAsync(
            new GraphQLRequest(
                @"
query CharactersQuery($skip: Int, $top: Int, $orderby: [Orderby]) {
    characters(count: true, skip: $skip, top: $top, orderby: $orderby) {
        id
        name
        strength
        dexterity
        constitution
        intelligence
        wisdom
        charisma
    }
}",
                new { skip = PageIndex * PageSize, top = PageSize }
            ),
            () => new { Characters = new List<Character>() }
        );

        count = response?.Extensions?["count"] as int?;
        pageCount = count / PageSize;
        characters = response?.Data.Characters;
    }

    private void PreviousPage()
    {
        if (Navigation is not null && HasPreviousPage)
            Navigation.NavigateTo(Navigation.GetUriWithQueryParameter(nameof(PageIndex), PageIndex - 1));
    }

    private void NextPage()
    {
        if (Navigation is not null && HasNextPage)
            Navigation.NavigateTo(Navigation.GetUriWithQueryParameter(nameof(PageIndex), PageIndex + 1));
    }
}
