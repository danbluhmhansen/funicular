namespace Funicular.Server.Data;

using Funicular.Server.Data.Models;

using Microsoft.AspNetCore.Identity.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore;

public class FunicularDbContext : IdentityDbContext<FunicularUser>
{
    public FunicularDbContext(DbContextOptions<FunicularDbContext> options) : base(options) { }

    protected FunicularDbContext() { }
}