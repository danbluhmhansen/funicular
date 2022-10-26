namespace Funicular.Server.Commands;

using Funicular.Server.Data;

public class UpdateEntity
{
    public UpdateEntity(FunicularDbContext db)
    {
        this.db = db;
    }

    private readonly FunicularDbContext db;

    public void Update<T>(T entity) where T : class => db.Update(entity);

    public void UpdateRange<T>(params T[] entities) where T : class => db.UpdateRange(entities);

    public void UpdateRange<T>(IEnumerable<T> entities) where T : class => db.UpdateRange(entities);
}