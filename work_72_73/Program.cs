using System;
using System.Collections.Generic;

public class SimpleCache<TKey, TValue> where TKey : notnull
{
    private static SimpleCache<TKey, TValue>?  _instance = null;
    private readonly Dictionary<TKey, TValue> _cache = new();
    private readonly object _lock = new();
    public readonly int Id =  1;

    public static SimpleCache<TKey, TValue> Create()
    {
        if (_instance != null) return _instance;
        else
        {
            _instance = new SimpleCache<TKey, TValue>();
            return _instance;
        }
    }

    private SimpleCache()
    {
        _cache = new Dictionary<TKey, TValue>();
        _lock = new object();
    }
    // Получает существующее значение или создаёт новое, если его нет.
    public TValue Get(TKey key, Func<TKey, TValue> factory)
    {
        lock (_lock)
        {
            if (_cache.TryGetValue(key, out var value)) return value;
            var newValue = factory(key);
            _cache[key] = newValue;
            return newValue;
        }
    }
    // Добавляет или обновляет значение по ключу.
    // Работает как dict.update() в Python.
    public void Add(TKey key, TValue value)
    {
        lock (_lock)
        {
            _cache[key] = value;
        }
    }
    // Попытка получить значение без создания нового.
    public bool TryGetValue(TKey key, out TValue? value)
    {
        lock (_lock)
        {
            return _cache.TryGetValue(key, out value!);
        }
    }
    // Удаляет элемент из кэша.
    public void Remove(TKey key)
    {
        lock (_lock)
        {
            _cache.Remove(key);
        }
    }
    // Очищает весь кэш.
    public void Clear()
    {
        lock (_lock)
        {
            _cache.Clear();
        }
    }
}
internal class Program
{
    static void Main()
    {
        SimpleCache<int, string> cache = SimpleCache<int, string>.Create();
        SimpleCache<int, string> simpleCache = SimpleCache<int, string>.Create();
        Console.WriteLine(cache.Id);
        Console.WriteLine(simpleCache.Id);
        // Добавляем или получаем значение
        string result1 = cache.Get(1, k => $"Value_{k}");
        Console.WriteLine(result1); // Value_1
        // Обновляем значение
        cache.Add(1, "NewValue");
        Console.WriteLine(cache.Get(1, k => "ShouldNotBeUsed")); // NewValue
        // Проверяем наличие
        if (cache.TryGetValue(2, out var val))   Console.WriteLine(val);
        else  Console.WriteLine("Key 2 not found"); // Key 2 not found
        // Удаляем
        cache.Remove(1);
        Console.WriteLine(cache.TryGetValue(1, out _)); // False
    }
}