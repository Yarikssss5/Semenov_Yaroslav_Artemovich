using System;

static double get_distance(double xA, double yA, double xB, double yB)
{
    return Math.Sqrt(Math.Pow(xA - xB, 2) + Math.Pow(yA - yB, 2));
}


static void task_2()
{
    double x1, y1, x2, y2, x3, y3;
    double x0, y0, r;

    // Читаем координату 1
    GetPoint1:
    Console.WriteLine("Введите координаты вершины 1 (x1 y1):");
    string input1 = Console.ReadLine() ?? "";
    string[] coords1 = input1.Split();

    if (coords1.Length != 2)
    {
        Console.WriteLine("Ошибка ввода. Попробуйте снова.");
        goto GetPoint1;
    }
    try
    {
        x1 = Convert.ToDouble(coords1[0]);
        y1 = Convert.ToDouble(coords1[1]);
    }
    catch
    {
        Console.WriteLine("Ошибка преобразования. Повторите.");
        goto GetPoint1;
    }

    while (true)
    {
        Console.WriteLine("Введите координаты вершины 2 (x2 y2):");
        string input2 = Console.ReadLine() ?? "";
        string[] coords2 = input2.Split();

        if (coords2.Length != 2)
        {
            Console.WriteLine("Неверное количество точек.");
            continue;
        }
        try
        {
            x2 = Convert.ToDouble(coords2[0]);
            y2 = Convert.ToDouble(coords2[1]);
            break;
        }
        catch
        {
            Console.WriteLine("Ошибка преобразования.");
            continue;
        }
    }

    while (true)
    {
        Console.WriteLine("Введите координаты вершины 3 (x3 y3):");
        string input3 = Console.ReadLine() ?? "";
        string[] coords3 = input3.Split();

        if (coords3.Length != 2)
        {
            Console.WriteLine("Неверное количество точек.");
            continue;
        }
        try
        {
            x3 = Convert.ToDouble(coords3[0]);
            y3 = Convert.ToDouble(coords3[1]);
            break;
        }
        catch
        {
            Console.WriteLine("Ошибка преобразования.");
            continue;
        }
    }

    GetCircle:
    Console.WriteLine("Введите координаты центра круга (x0 y0) и радиус r:");
    string circle = Console.ReadLine() ?? "";
    string[] circleValues = circle.Split();

    if (circleValues.Length != 3)
    {
        Console.WriteLine("Ошибка ввода. Повторите.");
        goto GetCircle;
    }
    try
    {
        x0 = Convert.ToDouble(circleValues[0]);
        y0 = Convert.ToDouble(circleValues[1]);
        r = Convert.ToDouble(circleValues[2]);
    }
    catch
    {
        Console.WriteLine("Ошибка преобразования.");
        goto GetCircle;
    }

    bool triangleInside = get_distance(x1, y1, x0, y0) <= r &&
                           get_distance(x2, y2, x0, y0) <= r &&
                           get_distance(x3, y3, x0, y0) <= r;

    double DistancePointToLine(double x, double y, double xA, double yA, double xB, double yB)
    {
        return Math.Abs((yB - yA) * x - (xB - xA) * y + xB * yA - yB * xA)
               / Math.Sqrt(Math.Pow(yB - yA, 2) + Math.Pow(xB - xA, 2));
    }

    bool circleInside = DistancePointToLine(x0, y0, x1, y1, x2, y2) >= r &&
                         DistancePointToLine(x0, y0, x2, y2, x3, y3) >= r &&
                         DistancePointToLine(x0, y0, x3, y3, x1, y1) >= r;

    if (triangleInside)
    {
        Console.WriteLine("Треугольник полностью внутри круга.");
    }
    else if (circleInside)
    {
        Console.WriteLine("Окружность полностью внутри треугольника.");
    }
    else
    {
        Console.WriteLine("Не наблюдается полного включения.");
    }
}

task_2();