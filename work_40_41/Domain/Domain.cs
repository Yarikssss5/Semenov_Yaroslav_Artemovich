namespace work40_41.Domain {
    public partial class Domain {
        public Domain() {
            while(true) {
                try {
                    Console.WriteLine("\n");
                    foreach(string el in solutions_texts.Keys) Console.WriteLine($"{el} - {solutions_texts[el]}");
                    Console.WriteLine("\nВведите номер задачи:");
                    string key = Console.ReadLine() ?? "";
                    if (key != "" & solutions_texts.ContainsKey(key)) {
                        if (solutions.ContainsKey(key)) {
                            if (input_handlers.ContainsKey(key)) {
                                object? user_params = input_handlers[key].DynamicInvoke(new Dictionary<string, object?>(){{"1", 1}});
                                object? call_result = solutions[key].DynamicInvoke(user_params);
                            } else Console.WriteLine("Для выбранной вами задачи не обнаружен обработчик для ввода данных пользователем");
                        } else {
                            Console.WriteLine("Для выбранной вами задачи пока что нет решения.");
                        }
                    } else {
                        Console.WriteLine("Вы ввели пустую строку или номер задачи отсутствуйщий в перечне");
                    }
                    Console.ReadKey();
                } catch (Exception e) {
                    Console.WriteLine(e.ToString());
                    Console.ReadKey();
                    continue;
                }
            }
        }
    }
}