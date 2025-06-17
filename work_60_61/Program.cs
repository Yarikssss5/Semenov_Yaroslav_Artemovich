
namespace work_60_61
{
    public class Person
    {
        public delegate int compareFunction(Person a, Person b);
        public string Name { get; set; }
        public int Age { get; set; }
        public double Height { get; set; }
        public bool IsActive { get; set; }
        public DateTime BirthDate { get; set; }
        public override string ToString() => $"Name: {Name}, Age: {Age}, Height: {Height:F2}, Active: {IsActive}, BirthDate: {BirthDate.ToShortDateString()}";
        
        public static List<Person> Sort(List<Person> list, compareFunction compare)
        {
            List<Person> sortedList = [..list];
            for (int i = 0; i < sortedList.Count - 1; i++)
            {
                for (int j = 0; j < sortedList.Count - i - 1; j++)
                {
                    if (compare(sortedList[j], sortedList[j + 1]) > 0)
                    {
                        Person temp = sortedList[j];
                        sortedList[j] = sortedList[j + 1];
                        sortedList[j + 1] = temp;
                    }
                }
            }
            return sortedList;
        }
        public static void PrintList(List<Person> list)
        {
            foreach (var person in list)
            {
                Console.WriteLine(person);
            }
        }
    }

    public delegate void Logger(params object[] values);
    class Program
    {
        static void Main(string[] args)
        {
            // Инициализация списка
            List<Person> people = new List<Person>
            {
                new Person { Name = "Алиса", Age = 30, Height = 165.5, IsActive = true, BirthDate = new DateTime(1994, 5, 15) },
                new Person { Name = "Майкл", Age = 25, Height = 180.2, IsActive = false, BirthDate = new DateTime(1999, 3, 22) },
                new Person { Name = "Арслан", Age = 35, Height = 175.8, IsActive = true, BirthDate = new DateTime(1989, 11, 7) },
                new Person { Name = "Вика", Age = 28, Height = 162.0, IsActive = false, BirthDate = new DateTime(1996, 8, 30) }
            };
            Console.WriteLine("До сортировки:");
            Person.PrintList(people);
            // Сортировка по возрасту
            Console.WriteLine("\nСортировка по возрасту:");
            var sortedByAge = Person.Sort(people, (a, b) => a.Age > b.Age ? 1 : 0);
            Person.PrintList(sortedByAge);
            // Сортировка по имени
            Console.WriteLine("\nСортировка по имени:");
            var sortedByName = Person.Sort(people, (a, b) => a.Name.Length > b.Name.Length ? 1 : 0);
            Person.PrintList(sortedByName);
            // Сортировка по дате рождения
            Console.WriteLine("\nСортировка по дате рождения:");
            var sortedByBirthDate = Person.Sort(people, (a, b) => a.BirthDate > b.BirthDate ? 1 : 0);
            Person.PrintList(sortedByBirthDate);
        }
    }
}