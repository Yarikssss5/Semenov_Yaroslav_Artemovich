// V 18 Yaroslav
// using System;
// using System.Collections.Generic;
// using System.Numerics;
// using System.Text;

// BigInteger factorial(BigInteger n)
// {
//     if (n == 0) return 1;
//     BigInteger result = 1;
//     for (BigInteger i = 2; i <= n; i++) result *= i;
//     return result;
// }

// void SwapForList(int first, int other, List<BigInteger> n)
// {
//     var tmp = n[first];
//     n[first] = n[other];
//     n[other] = tmp;
// }

// void GnomeSort(List<BigInteger> list)
// {
//     for (int i = 1; i < list.Count;)
//     {
//         if (i == 0 || list[i] >= list[i - 1]) i++;
//         else
//         {
//             SwapForList(i, i - 1, list);
//             i--;
//         }
//     }
// }

// List<BigInteger> array = new List<BigInteger>();
// string? uiBuf = null;
// Console.WriteLine("Введите количество элементов :");
// uiBuf = Console.ReadLine();

// if (int.TryParse(uiBuf, out int i_user_input))
// {
//     for (int i = 0; i < i_user_input; i++)
//     {
//         BigInteger C = factorial(i_user_input) / (factorial(i) * factorial(i_user_input - i));
//         array.Add(C);
//     }

//     StringBuilder sb = new();
//     Console.WriteLine("Массив до сортировки :");
//     foreach (var i in array) sb.Append($"{i} , ");
//     Console.WriteLine(sb.ToString());
//     sb.Clear();
//     GnomeSort(array);
//     Console.WriteLine("Массив после сортировки :");
//     foreach (var i in array) sb.Append($"{i} , ");
//     Console.WriteLine(sb.ToString());
// }


// V  Arslan
using System.Numerics;

namespace work_14_15 {

    public struct Item
    {
        public BigInteger N;
        public BigInteger K;
        public BigInteger Value;
        public Item(BigInteger n, BigInteger k, BigInteger value)
        {
            N = n;
            K = k;
            Value = value;
        }

        public override string ToString()
        {
            return $"N = {N}, K = {K}, Value = {Value}";
        }
    }
    internal class Program
    {
        static void SwapForList(int first, int other, List<BigInteger> n)
        {
            var tmp = n[first];
            n[first] = n[other];
            n[other] = tmp;
        }        
        static void GnomeSort(List<BigInteger> list)
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
        private static BigInteger factorial(BigInteger n)
        {
            if (n == 0) return 1;
            BigInteger result = 1;
            for (BigInteger i = 2; i <= n; i++) result *= i;
            return result;
        }

        private static BigInteger BinomialCoefficient(BigInteger n, BigInteger k)
        {
            if (k > n || k < 0)
                return 0;


            BigInteger result = 1;
            for (BigInteger i = 0; i < k; i++)
            {
                result *= (n - i);
                result /= (i + 1);
            }

            return result;
        }

        private static void Main(string[] args)
        {
            List<Item> items = new List<Item>();
            string? uiBuf = null;
            Console.WriteLine("Введите количество элементов :");
            uiBuf = Console.ReadLine();
            BigInteger PlusNum = 0;
            BigInteger Result = 0;
            if (int.TryParse(uiBuf, out int i_user_input))
            {

                int M = 2; // или можно спросить у пользователя
                int N = i_user_input;

                for (int k = 0; k <= N; k++)
                {
                    BigInteger n = N + k;
                    BigInteger C = BinomialCoefficient(n, M);
                    
                    if (C % 2 == 0)
                        items.Add(new Item(n, k, C));
                }
            }
            Console.WriteLine("\n");
            foreach (var i in items) Console.WriteLine($"N = {i.N} , K = {i.K} , Value = {i.Value} ,");
            Console.WriteLine("\n");
            var result = items.OrderBy(item => item.Value).ToList();
            Console.WriteLine("Максимальный элемент : ");
            Console.WriteLine(result[result.Count -1]);
        }
    }
}