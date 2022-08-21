namespace Funicular.Server.Data;

using System;

using Funicular.Server.Data.Models;

using Microsoft.EntityFrameworkCore;

internal class FunicularDbContext : DbContext
{
    public FunicularDbContext(DbContextOptions options) : base(options) { }

    protected FunicularDbContext() { }

    public DbSet<Character> Characters => Set<Character>();
    public DbSet<CharacterField> CharacterFields => Set<CharacterField>();

    protected override void OnModelCreating(ModelBuilder modelBuilder)
    {
        if (modelBuilder is null) throw new ArgumentNullException(nameof(modelBuilder));

        base.OnModelCreating(modelBuilder);

        modelBuilder.Entity<CharacterField>().HasKey(field => field.Name);
    }
}
