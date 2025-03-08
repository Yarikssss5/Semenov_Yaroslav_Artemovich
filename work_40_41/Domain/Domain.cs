namespace work_40_41.Domain {
    public partial class Domain {
        public Domain() {
            try {
                Console.WriteLine("\n");
                foreach(string el in solutions_texts.Keys) Console.WriteLine($"{el} - {solutions_texts[el]}");
                string key = inputTaskNumber();
                object? user_params = input_handlers[key].DynamicInvoke(key);
                object? call_result = solutions[key].DynamicInvoke(user_params);
                Console.ReadKey();
            } catch (Exception e) {
                Console.WriteLine(e.ToString());
                Console.ReadKey();
            }
        }
    }
}