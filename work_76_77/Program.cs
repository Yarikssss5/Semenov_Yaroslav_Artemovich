using System;

namespace App
{
interface IEnemyBehavior
{
    void Action();
}

public class RunBehavior : IEnemyBehavior
{
    public void Action()
    {
        Console.WriteLine("Я убегаю");
    }
}

public class AgressiveBehavior : IEnemyBehavior
{
    public void Action()
    {
        Console.WriteLine("Я нападаю");
    }
}

public class NeutralBehaviour : IEnemyBehavior
{
    public void Action()
    {
        Console.WriteLine("Привет");
    }
}

public class Person
{
    private IEnemyBehavior _behavior;

    public int Health { get; set; }
    public bool PlayerNear { get; set; }

    public Person(int health, bool playerNear)
    {
        Health = health;
        PlayerNear = playerNear;
        defineBehavior();
    }

    public void defineBehavior()
    {
        if (!PlayerNear) _behavior = new NeutralBehaviour();
        else if (Health > 70) _behavior = new AgressiveBehavior();
        else if (Health > 30) _behavior = new NeutralBehaviour();
        else _behavior = new RunBehavior();
    }

    public void Act()
    {
        defineBehavior(); 
        _behavior.Action();
    }
}

public class Program
{
    static void Main()
    {
        Person enemy = new(health: 100, playerNear: true);

        while (true)
        {
            Console.WriteLine("Введите расстояние до врага (1(Близко) / 2(Далеко):");
            string distance = Console.ReadLine() ?? "";

            Console.WriteLine("Введите здоровье врага (0-100):");
            string input = Console.ReadLine() ?? "";
            int.TryParse(input, out int health);

            enemy.Health = health;
            enemy.PlayerNear = distance == "1";

            enemy.Act();

            Console.WriteLine();
        }
    }
}
}
