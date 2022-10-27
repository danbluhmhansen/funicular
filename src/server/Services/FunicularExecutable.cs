namespace Funicular.Server.Services;

using Funicular.Server.Data;

public class FunicularExecutable<T> : EntityFrameworkExecutable<T> where T : class
{
    public FunicularExecutable(FunicularDbContext db) : base(db.Set<T>()) { }
}