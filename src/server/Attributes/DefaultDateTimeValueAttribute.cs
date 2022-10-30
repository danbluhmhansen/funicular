namespace Funicular.Server.Attributes;

using System.Reflection;

using HotChocolate.Types.Descriptors;

[AttributeUsage(AttributeTargets.Parameter, Inherited = false, AllowMultiple = true)]
public sealed class DefaultDateTimeValueAttribute : DescriptorAttribute
{
    public DateTime Value { get; }

    public DefaultDateTimeValueAttribute(int year, int month, int day) : this(year, month, day, 0, 0, 0) { }

    public DefaultDateTimeValueAttribute(int year, int month, int day, int hour, int minute, int second)
    {
        Value = new(year, month, day, hour, minute, second);
    }

    public DefaultDateTimeValueAttribute(long ticks)
    {
        Value = new(ticks);
    }

    public DefaultDateTimeValueAttribute()
    {
        Value = DateTime.UtcNow;
    }

    protected override void TryConfigure(
        IDescriptorContext context,
        IDescriptor descriptor,
        ICustomAttributeProvider element
    )
    {
        (descriptor as IArgumentDescriptor)?.DefaultValue(Value);
        (descriptor as IDirectiveArgumentDescriptor)?.DefaultValue(Value);
        (descriptor as IInputFieldDescriptor)?.DefaultValue(Value);
    }
}