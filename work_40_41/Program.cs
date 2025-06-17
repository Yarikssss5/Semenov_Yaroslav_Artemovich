namespace work_40_41 {

    class FSReader {
        public static List<string> ReadLines(string fullpath) {
            List<string> result = new();
            try {
                if (File.Exists(fullpath)) {
                    using (StreamReader sr = new (fullpath)) {
                        string? buf = sr.ReadLine();
                        while (buf != null) {
                            result.Add(buf.ToString());
                            buf = sr.ReadLine();
                        }
                    }
                } else Console.WriteLine($"File does not exists ! {fullpath}");
            } catch (Exception e) {
                Console.WriteLine(e.ToString());
            }
            return result;
        }
    }

    class FSWriter {
        public static void WriteLines(List<string> lines, string filepath) {
            try {
            if (File.Exists(filepath)) {
                File.Delete(filepath);
            }
            using (StreamWriter sw = new(filepath)) {
                foreach(var line in lines) {
                    sw.WriteLine(line);
                }
            }
            } catch (Exception e) {
                Console.WriteLine(e.ToString());
            }
        }
    }


    class StudentReader {
        static 
    }

    internal class Program {
        private static void Main() {

        }
    }
}
