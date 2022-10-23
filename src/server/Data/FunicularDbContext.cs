namespace Funicular.Server.Data;

using Funicular.Server.Data.Models;

using Microsoft.AspNetCore.Identity.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore;

public class FunicularDbContext : IdentityDbContext<FunicularUser>
{
    public FunicularDbContext(DbContextOptions<FunicularDbContext> options) : base(options) { }

    protected FunicularDbContext() { }

    public DbSet<WeatherForecast> WeatherForecasts => Set<WeatherForecast>();

    protected override void OnModelCreating(ModelBuilder builder)
    {
        base.OnModelCreating(builder);

        builder.Entity<WeatherForecast>(b =>
        {
            b.HasKey(_ => _.Date);
            b.Property(_ => _.Date).HasColumnType("date");
        });
    }
}