namespace work_30_31 {
    internal class Program {
        static void Main() {
            int[] arr = [ 1, 3, 4, 7, 89 ];
            foreach (int i in arr) Console.Write($" {i}");
            Console.WriteLine("");
            //arr = sort_first(arr);
            arr = sort_second(arr);
            foreach (int i in arr) Console.Write($" {i}");
        }

        static int[] sort_first(int[] array) {
            int tmp = 0;
            bool check = false;
            int length = array.Length;
            while (true)
            {
                for (int i = 0; i < length; i++)
                {
                    if (i + 1 < array.Length)
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


        static void get_index_of_min(int [] arr, int ind) {
            int min;
            
        }
        static int[] sort_second(int[] array) {
            for (int i = 1; i < array.Length; i ++ ) {

            }
            return array;
        }
    }
}
