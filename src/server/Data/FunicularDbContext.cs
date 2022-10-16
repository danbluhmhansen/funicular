namespace Funicular.Server.Data;

using Funicular.Server.Data.Models;

using Microsoft.AspNetCore.Identity.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore;

internal class FunicularDbContext : IdentityDbContext<FunicularUser>
{
    public FunicularDbContext(DbContextOptions<FunicularDbContext> options) : base(options) { }

    protected FunicularDbContext() { }

    public DbSet<Character> Characters => Set<Character>();
    public DbSet<DynamicField> CharacterFields => Set<DynamicField>();

    protected override void OnModelCreating(ModelBuilder modelBuilder)
    {
        base.OnModelCreating(modelBuilder);

        modelBuilder.Entity<DynamicField>().HasKey(field => field.Name);
    }
}