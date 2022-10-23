namespace Funicular.Server.Controllers;

using System.Collections.Generic;
using System.Threading.Tasks;

using Funicular.Server.Data;
using Funicular.Server.Data.Models;

using Microsoft.AspNetCore.Mvc;
using Microsoft.EntityFrameworkCore;

[ApiController]
[Route("[controller]")]
public class WeatherForecastsController : ControllerBase
{
    public WeatherForecastsController(FunicularDbContext db)
    {
        this.db = db;
    }

    private readonly FunicularDbContext db;

    [HttpGet]
    public async Task<ActionResult<IEnumerable<WeatherForecast>>> Get() => Ok(await db.WeatherForecasts.ToListAsync());
}