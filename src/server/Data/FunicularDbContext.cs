namespace Funicular.Server.Data;

using Funicular.Server.Data.Models;

using Microsoft.AspNetCore.Identity.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore;

public class FunicularDbContext : IdentityDbContext<FunicularUser>
{
    public FunicularDbContext(DbContextOptions<FunicularDbContext> options) : base(options) { }

    protected FunicularDbContext() { }

    public DbSet<Character> Characters => Set<Character>();
    public DbSet<WeatherForecast> WeatherForecasts => Set<WeatherForecast>();

    protected override void ConfigureConventions(ModelConfigurationBuilder builder)
    {
        base.ConfigureConventions(builder);

        builder.Properties<CharacterId>().HaveConversion<CharacterId.EfCoreValueConverter>();
    }

    protected override void OnModelCreating(ModelBuilder builder)
    {
        base.OnModelCreating(builder);

        builder.Entity<Character>().Property(_ => _.Ints).HasColumnType("jsonb");

        builder.Entity<WeatherForecast>(b =>
        {
            b.HasKey(_ => _.Date);
            b.Property(_ => _.Date).HasColumnType("date");
        });
    }
}