using System.Text;

namespace work_36_37_char {
    internal class Program {
        private static readonly Dictionary<string, string> tasks_ = new Dictionary<string, string>(){
            {"1", "заменяет все вхождения одного символа на другой в заданной строке"},
            {"2", "генерирует 32-х значную строку из случайного набора символов"},
            {"3", "подсчитывает количество гласных и согласных букв в строке"},
            {"4", "Проверить задачу (3) используя задачу (2)"},
            {"q", "Выход"}
        };
        private static string task_1(string _line, char _replace_symbov, char _sybow_for_replace) {
            // заменяет все вхождения одного символа на другой в заданной строке :
            StringBuilder sb = new StringBuilder();
            foreach(char i in _line) sb.Append(i != _replace_symbov ? i : _sybow_for_replace);
            return sb.ToString();
        }
        private static string task_2() {
            // генерирует 32-х значную строку из случайного набора символов :
            try{
                Random myrgn = new Random(); Random rng_2 = new Random();
                Random rng_3 = new Random(); Random rng_4 = new Random();
                Random rng_5 = new Random(); StringBuilder sb = new StringBuilder();
                int rand_num = 0;
                int sym_counter = 0;
                for (int i = 0; i < 32; i++) {
                    rand_num = myrgn.Next(0, 4);
                    switch(rand_num) {
                        case 0:
                            sb.Append((char)rng_2.Next(64, 91)); 
                            sym_counter++;
                            break;
                        case 1:
                            sb.Append((char)rng_3.Next(96, 123));
                            sym_counter++;
                            break;
                        case 2:
                            sb.Append((char)rng_4.Next(47, 58)); 
                            sym_counter++;
                            break;
                        case 3:
                            sb.Append((char)rng_5.Next(31, 127));
                            sym_counter++;
                            break;
                    }
                };
                return sb.ToString();
            } catch(Exception e) {Console.WriteLine(e.ToString()); return "";}
        }
        private static List<int> task_3(string _line) {
            // v 4. подсчитывает количество гласных и согласных букв в строке :
            try {
                _line = _line.ToLower();
                string vowels_template = "aeiouyаеиёиоуыэюя";
                string consonant_template = "bcdfghjklmnpqrstvwxyzбвгджзйклмнпрстфхцчшщ";
                int vowels = 0;
                int consonant = 0;
                foreach(char mychar in _line) {
                    if (vowels_template.Contains(mychar)) vowels++;
                    else if(consonant_template.Contains(mychar)) consonant++;
                }
                return [consonant, vowels];
            } catch (Exception e) {Console.WriteLine(e.ToString()); return [0, 0];}
        }
        private static void Main() {
            bool exit = false;
            StringBuilder sb = new StringBuilder();
            string task = "";
            char replace_symbol = ' ';
            string replaced_line = "";
            while(!exit) {
                foreach(var i in tasks_.Keys)Console.WriteLine($"{i} - {tasks_[i]}");
                task = Console.ReadLine() ?? "";
                if (task == "") continue;
                if (!tasks_.ContainsKey(task)) continue;
                switch(task) {
                    case "1":
                        sb.Clear();
                        Console.WriteLine("\nВведите строку : ");
                        sb.Append(Console.ReadLine() ?? "");
                        Console.WriteLine("\nВведите символ который хотите поменять : ");
                        replace_symbol = Console.ReadKey().KeyChar;
                        Console.WriteLine("\nВведите символ на который хотите поменять : ");
                        char replaycement_symbol = Console.ReadKey().KeyChar;
                        replaced_line = task_1(sb.ToString(),  replace_symbol, replaycement_symbol);
                        Console.WriteLine($"\nИтоговая строка {replaced_line}");
                        break;
                    case "2":
                        Console.WriteLine($"Итоговая строка : {task_2()}");
                        break;
                    case "3":
                        Console.WriteLine("Введите строку: ");
                        string line = Console.ReadLine() ?? "";
                        if (line == "") Console.WriteLine("Гласных: 0, Согласных: 0");
                        List<int> result = task_3(line);
                        Console.WriteLine($"Гласных: {result[0]}, Согласных: {result[1]}");
                        break;
                    case "4":
                        sb.Clear();
                        sb.Append(task_2());
                        Console.WriteLine($"Строка : {sb}");
                        Console.WriteLine("\nВведите символ который хотите поменять : ");
                        replace_symbol = Console.ReadKey().KeyChar;
                        Console.WriteLine("\nВведите символ на который хотите поменять : ");
                        replaycement_symbol = Console.ReadKey().KeyChar;
                        replaced_line = task_1(sb.ToString(),  replace_symbol, replaycement_symbol);
                        Console.WriteLine($"\nИзменённая строка : {replaced_line}");
                        break;
                    case "q":
                        exit = true;
                        break;
                }
                Console.ReadLine();
                continue;
            }
        }
    }
}
