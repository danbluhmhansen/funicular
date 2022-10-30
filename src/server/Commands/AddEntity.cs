namespace Funicular.Server.Commands;

using Funicular.Server.Data;

public class AddEntity
{
    public AddEntity(FunicularDbContext db)
    {
        this.db = db;
    }

    private readonly FunicularDbContext db;

    public void Add<T>(T entity) where T : class => db.Add(entity);

    public void AddRange<T>(params T[] entities) where T : class => db.AddRange(entities);

    public void AddRange<T>(IEnumerable<T> entities) where T : class => db.AddRange(entities);
}