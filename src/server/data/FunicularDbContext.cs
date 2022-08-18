namespace Funicular.Server.Data;

using Funicular.Server.Data.Models;

using Microsoft.EntityFrameworkCore;

internal class FunicularDbContext : DbContext
{
    public FunicularDbContext(DbContextOptions options) : base(options) { }

    protected FunicularDbContext() { }

    public DbSet<Character> Characters => Set<Character>();
}
