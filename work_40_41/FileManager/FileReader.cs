namespace work40_41.FileManager {
    class FileReader {
        private string filepath;
        public FileReader(string? _filepath) {
            filepath = _filepath ?? "";
            if (filepath != "" & filepath != null) {
                if (!File.Exists(filepath)) {
                    throw new Exception($"Файла несуществует : ${filepath}");
                }
            } else {
                Console.WriteLine("Путь пустой !");
                filepath = "";
            }
        }
        public string[]? ReadLines() {
            string[] lines = [];
            string line = "";
            try {
                using (StreamReader sr = new StreamReader(filepath)) {
                    while((line = sr.ReadLine() ?? "") != null) {
                        string nonnull_s = line ?? "";
                        if (nonnull_s != "") {
                            lines.Append(nonnull_s);
                        }
                    }
                }
                return lines;
            } catch (Exception e) {
                Console.WriteLine(e.ToString());
                return null;
            }
        }
    }
}