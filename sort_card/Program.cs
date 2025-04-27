
using System.Reflection.Metadata;

namespace app
{
    internal class Program
    {
        static List<char> TYPES = new List<char> { '♠', '♣', '♥', '♦' };
        static List<char> KERM = new List<char> { 'J', 'Q', 'K', 'A' };

        const char PIKI = '♠';
        const char CRESTI = '♣';
        const char CHERVY = '♥';
        const char BUBI = '♦';

        static string FORMAT = "";

        public static void Main()
        {
            List<int> PIKI_buf = new List<int> { };
            List<int> CHERVY_buf = new List<int> { };
            List<int> BUBU_buf = new List<int> { };
            List<int> CRESTY_buf = new List<int> { };
            string john = "♦6♥2♣3♦5♠J♣Q♠K♣7♦2♣5♥5♥10♠A";
            string uncle = "♠2♠3♠5♥J♥Q♥K♣8♣9♣10♦4♦5♦6♦7";

            int tmp = 0;

            // Достаём из колоды порядок мастей :
            for (int i = 0; i < uncle.Length; i++)
            {
                if (TYPES.Contains(uncle[i]))
                {
                    if (!FORMAT.Contains(uncle[i])) FORMAT += uncle[i];

                    if (int.TryParse(uncle[i + 1].ToString(), out int OI))
                    {
                        if (OI == 1) tmp = 10;
                        else tmp = OI;
                    }
                    else
                    {
                        // Encode :
                        if (uncle[i + 1] == 'J') tmp = 11;
                        else if (uncle[i + 1] == 'Q') tmp = 12;
                        else if (uncle[i + 1] == 'K') tmp = 13;
                        else if (uncle[i + 1] == 'A') tmp = 14;
                        else Console.WriteLine("Такой карты несуществует");
                    }

                    switch (uncle[i])
                    {
                        case PIKI:
                            PIKI_buf.Add(tmp);
                            break;

                        case CRESTI:
                            CRESTY_buf.Add(tmp);
                            break;

                        case CHERVY:
                            CHERVY_buf.Add(tmp);
                            break;

                        case BUBI:
                            BUBU_buf.Add(tmp);
                            break;
                        case '0':
                            break;
                    }
                }
            }
            Console.WriteLine("Порядок мастей дяди : ");
            Console.WriteLine($"{FORMAT}");

            List<int> john_PIKI = new List<int> { };
            List<int> john_BUBI = new List<int> { };
            List<int> john_CRESTI = new List<int> { };
            List<int> john_CHERVY = new List<int> { };

            // Clean up :
            tmp = 0;

            // Patsing string :
            for (int i = 0; i < john.Length; i++)
            {
                // Мы не выйдем за границу массива так как опорный элемент у нас это масть:
                if (TYPES.Contains(john[i]))
                {
                    if (int.TryParse(john[i + 1].ToString(), out int OI))
                    {
                        if (OI == 1) tmp = 10;
                        else tmp = OI;
                    }
                    else
                    {
                        // Encode :
                        if (john[i + 1] == 'J') tmp = 11;
                        else if (john[i + 1] == 'Q') tmp = 12;
                        else if (john[i + 1] == 'K') tmp = 13;
                        else if (john[i + 1] == 'A') tmp = 14;
                        else Console.WriteLine("Такой карты несуществует");
                    }

                    switch (john[i])
                    {
                        case PIKI:
                            john_PIKI.Add(tmp);
                            break;

                        case CRESTI:
                            john_CRESTI.Add(tmp);
                            break;

                        case CHERVY:
                            john_CHERVY.Add(tmp);
                            break;

                        case BUBI:
                            john_BUBI.Add(tmp);
                            break;
                        case '0':
                            break;
                    }
                }
            }

            john_BUBI = my_sort(john_BUBI);
            john_CHERVY = my_sort(john_CHERVY);
            john_CRESTI = my_sort(john_CRESTI);
            john_PIKI = my_sort(john_PIKI);

            Console.WriteLine(" ");
            Console.WriteLine("Отсортированные карты Джона : ");
            string res = "";
            for (int i = 0; i < FORMAT.Length; i++)
            {
                switch (FORMAT[i])
                {
                    case PIKI:
                        foreach (int j in john_PIKI)
                        {
                            res = (j > 10) ? decode(j).ToString() : j.ToString();
                            Console.Write($" {PIKI}{res}");
                        }
                        break;

                    case CRESTI:
                        foreach (int j in john_CRESTI)
                        {
                            res = (j > 10) ? decode(j).ToString() : j.ToString();
                            Console.Write($" {CRESTI}{res}");
                        }
                        break;

                    case CHERVY:
                        foreach (int j in john_CHERVY)
                        {
                            res = (j > 10) ? decode(j).ToString() : j.ToString();
                            Console.Write($" {CHERVY}{res}");
                        }
                        break;

                    case BUBI:
                        foreach (int j in john_BUBI)
                        {
                            res = (j > 10) ? decode(j).ToString() : j.ToString();
                            Console.Write($" {BUBI}{res}");
                        }
                        break;
                }
            }
            Console.WriteLine("");

        }
        public static char decode(int code)
        {
            char res = ' ';
            if (code == 11) res = 'J';
            else if (code == 12) res = 'Q';
            else if (code == 13) res = 'K';
            else if (code == 14) res = 'A';
            return res;
        }

        static List<int> my_sort(List<int> array)
        {
            int tmp = 0;
            bool check = false;
            int length = array.Count;
            while (true)
            {
                for (int i = 0; i < length; i++)
                {
                    if (i + 1 < array.Count)
                    {
                        if (array[i + 1] > array[i])
                        {
                            tmp = array[i];
                            array[i] = array[i + 1];
                            array[i + 1] = tmp;
                        }
                    }
                }
                length --;
                if (length == 0) break;
                if (check) break;
            }
            return array;
        }
    }
}