using System.Text;

namespace work_40_41.Domain {
   public partial class Domain {
        private static object? solution_1(Dictionary<string, object?> _params) {
            if (!validate_dict(_params)) return null;
            List<string[]> data = (List<string[]>?)_params["lines"] ?? [];
            string paramName = (string?)_params["param"] ?? "";
            if (!_params.ContainsKey(paramName)) return null;
            string param = (string?)_params[paramName] ?? "";
            string filePath = (_params["filepath"] ?? "").ToString() ?? "";
            var fr = new FileManager.FileManager();
            var readed_data = fr.Read(filePath);
            var parsed_data = parseReadedLines(readed_data);

            List<Dictionary<string, object?>> selectedLines = new List<Dictionary<string, object?>>();
            foreach (var el in parsed_data ?? []) if ((el[paramName] ?? "").ToString() == param) selectedLines.Add(el);
            foreach (var el in selectedLines)
            {
                StringBuilder sb = new StringBuilder();
                sb.Append(selectedLines.IndexOf(el).ToString());
                foreach (string fieldName in paramsNames)
                {
                    string sFieldName = field_translations[fieldName] ?? "";
                    object sFieldValue = el[fieldName] ?? "";
                    sb.Append($"{sFieldName}: {sFieldValue.ToString()}");
                }
                Console.WriteLine(sb.ToString());
            }
            return null;
        }

        private static void solution_2(Dictionary<string, object?> _params)
        {

        }

        private static readonly Dictionary<string, Delegate> solutions = new Dictionary<string, Delegate>(){
            {"1", solution_1},
            {"2", solution_2}
        };
    }
}