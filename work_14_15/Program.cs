// V 18 Yaroslav
using System.Collections.Generic;
using System.Numerics;
using System.Text;

BigInteger factorial(BigInteger n)
{
    if (n == 0) return 1;
    BigInteger result = 1;
    for (BigInteger i = 2; i <= n; i++) result *= i;
    return result;
}

void SwapForList(int first, int other, List<BigInteger> n)
{
    var tmp = n[first];
    n[first] = n[other];
    n[other] = tmp;
}

void GnomeSort(List<BigInteger> list)
{
    for (int i = 1; i < list.Count;)
    {
        if (i == 0 || list[i] >= list[i - 1]) i++;
        else
        {
            SwapForList(i, i - 1, list);
            i--;
        }
    }
}

List<BigInteger> array = new List<BigInteger>();
string? uiBuf = null;
Console.WriteLine("Введите количество элементов :");
uiBuf = Console.ReadLine();

if (int.TryParse(uiBuf, out int i_user_input))
{
    for (int i = 0; i < i_user_input; i++)
    {
        BigInteger C = factorial(i_user_input) / (factorial(i) * factorial(i_user_input - i));
        array.Add(C);
    }

    StringBuilder sb = new();
    Console.WriteLine("Массив до сортировки :");
    foreach (var i in array) sb.Append($"{i} , ");
    Console.WriteLine(sb.ToString());
    sb.Clear();
    GnomeSort(array);
    Console.WriteLine("Массив после сортировки :");
    foreach (var i in array) sb.Append($"{i} , ");
    Console.WriteLine(sb.ToString());
}


// B


void PrintArray(int[] arr) => Console.WriteLine(string.Join(" ", arr));
int[] array = { -5, 3, -1, 8, 2, -7, 0, 4 };
Console.WriteLine("Исходный массив:");
PrintArray(array);
var positiveNumbers = new List<int>();
foreach (var i in array) if (i > 0) positiveNumbers.Add(i);
int index = 0;
for (int i = 0; i < array.Length; i++)
{
    if (array[i] > 0)
    {
        array[i] = positiveNumbers[index];
        index++;
    }
}
Console.WriteLine("Массив после сортировки положительных элементов:");
PrintArray(array);