class Program
{
    static void Main()
    {
        Console.WriteLine("Введите количество чисел для сортировки: ");                                                                                                                                                                                         
        int n;
        int.TryParse(Console.ReadLine(), out n);
        int[] array = new int[n];
        int index = 0;
        Console.WriteLine("Введите числа для сортировки: ");
        while (index < n)
        {
            if (int.TryParse(Console.ReadLine(), out int value))
            {
                array[index] = value;
                index++;
            }
            else
            {
                Console.WriteLine("Ошибка: введите целое число.");
            }
        }
        for (int i = 0; i < array.Length - 1; i++)
        {
            int maxIndex = i;
            for (int j = i + 1; j < array.Length; j++)
            {
                if (array[j] > array[maxIndex])
                    maxIndex = j;
            }
            if (maxIndex != i)
            {
                int temp = array[i];
                array[i] = array[maxIndex];
                array[maxIndex] = temp;
            }
        }
        Console.WriteLine("Отсортированный массив:");
        foreach (int num in array) Console.Write($"{num} ");
        Console.WriteLine();
    }
}
