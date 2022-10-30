namespace Funicular.Server.Commands;

using Funicular.Server.Data;

public class RemoveEntity
{
    public RemoveEntity(FunicularDbContext db)
    {
        this.db = db;
    }

    private readonly FunicularDbContext db;

    public void Remove<T>(T entity) where T : class => db.Remove(entity);

    public void RemoveRange<T>(params T[] entities) where T : class => db.RemoveRange(entities);

    public void RemoveRange<T>(IEnumerable<T> entities) where T : class => db.RemoveRange(entities);
}