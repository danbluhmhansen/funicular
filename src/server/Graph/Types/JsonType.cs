namespace Funicular.Server.Graph.Types;

using System.Text.Json;

using HotChocolate.Language;

public class JsonType : ScalarType
{
    /// <summary>
    /// Initializes a new instance of the <see cref="JsonType"/> class.
    /// </summary>
    public JsonType() : base("Json") { }

    /// <summary>
    /// Initializes a new instance of the <see cref="JsonType"/> class.
    /// </summary>
    public JsonType(string name, string? description = null, BindingBehavior bind = BindingBehavior.Explicit)
        : base(name, bind)
    {
        Description = description;
    }

    /// <inheritdoc/>
    public override Type RuntimeType => typeof(JsonElement);

    /// <inheritdoc/>
    public override bool IsInstanceOfType(IValueNode literal)
    {
        if (literal is null)
            throw new ArgumentNullException(nameof(literal));

        return literal switch
        {
            ObjectValueNode or NullValueNode => true,
            _ => false,
        };
    }

    /// <inheritdoc/>
    public override object? ParseLiteral(IValueNode valueSyntax)
    {
        return valueSyntax switch
        {
            StringValueNode svn => TryDeserializeFromString(svn.Value),
            NullValueNode => null,
            _ => throw new NotSupportedException(),
        };
    }

    private static object TryDeserializeFromString(string? serialized)
    {
        try
        {
            return JsonDocumentConverter.Convert(JsonSerializer.Deserialize<object>(serialized));
        }
        catch
        {
            throw new NotSupportedException();
        }
    }

    /// <inheritdoc/>
    public override IValueNode ParseValue(object? runtimeValue)
    {
        return runtimeValue switch
        {
            null => NullValueNode.Default,

            IReadOnlyDictionary<string, object> dict => ParseValue(dict, new HashSet<object>()),

            JsonElement jsonElement => ParseValue(jsonElement, new HashSet<object>()),

            _ => throw new NotSupportedException(),
        };
    }

    private IValueNode ParseValue(object? value, ISet<object> set)
    {
        if (value is null)
        {
            return NullValueNode.Default;
        }

        switch (value)
        {
            case string s:
                return new StringValueNode(s);
            case short s:
                return new IntValueNode(s);
            case int i:
                return new IntValueNode(i);
            case long l:
                return new IntValueNode(l);
            case float f:
                return new FloatValueNode(f);
            case double d:
                return new FloatValueNode(d);
            case decimal d:
                return new FloatValueNode(d);
            case bool b:
                return new BooleanValueNode(b);
            case Guid g:
                return new StringValueNode(g.ToString());
            case DateTimeOffset d:
                return new StringValueNode(d.ToString());
        }

        if (set.Add(value))
        {
            if (value is IReadOnlyDictionary<string, object> dict)
            {
                var fields = new List<ObjectFieldNode>();
                foreach (KeyValuePair<string, object> field in dict)
                {
                    fields.Add(new ObjectFieldNode(field.Key, ParseValue(field.Value, set)));
                }
                return new ObjectValueNode(fields);
            }

            return ParseValue(JsonDocumentConverter.Convert(value), set);
        }

        throw new NotSupportedException();
    }

    /// <inheritdoc/>
    public override IValueNode ParseResult(object? resultValue) => ParseValue(resultValue);

    /// <inheritdoc/>
    public override bool TrySerialize(object? runtimeValue, out object? resultValue)
    {
        if (runtimeValue == null)
        {
            resultValue = null;
            return true;
        }

        return Convert(runtimeValue, out resultValue);
    }

    /// <inheritdoc/>
    public override bool TryDeserialize(object? resultValue, out object? runtimeValue)
    {
        return Convert(resultValue, out runtimeValue);
    }

    private bool Convert(object? resultValue, out object? runtimeValue)
    {
        runtimeValue = null;
        switch (resultValue)
        {
            case IDictionary<string, object> dictionary:
                {
                    var result = new Dictionary<string, object?>();
                    foreach (KeyValuePair<string, object> element in dictionary)
                    {
                        if (Convert(element.Value, out var elementValue))
                        {
                            result[element.Key] = elementValue;
                        }
                        else
                        {
                            return false;
                        }
                    }

                    runtimeValue = result;
                    return true;
                }
            case IValueNode literal:
                runtimeValue = ParseLiteral(literal);
                return true;
            default:
                runtimeValue = JsonDocumentConverter.Convert(resultValue);
                return true;
        }
    }
}