namespace Funicular.Server.Graph;

using Funicular.Server.Data.Models;
using Funicular.Server.Graph.Types;

public class FunicularQuery
{
    [UsePaging(DefaultPageSize = 100, MaxPageSize = 100), UseFiltering, UseSorting]
    public IExecutable<WeatherForecast> GetWeatherForecasts(IExecutable<WeatherForecast> executable) => executable;

    [UsePaging, UseFiltering(typeof(CharacterFilterType)), UseSorting(typeof(CharacterSortType))]
    public IExecutable<Character> GetCharacters(IExecutable<Character> executable) => executable;
}