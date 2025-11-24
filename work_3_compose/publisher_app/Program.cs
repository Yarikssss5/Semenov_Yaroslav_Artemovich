using System.Globalization;
using System.Text;
using System.Text.Json;
using RabbitMQ.Client;

class Lesson
{
    public int Count {get; set; }
    public string Office {get; set;}
    public string Teacher {get; set;}
    public string Subject {get; set;}
    public string Time {get; set;}

    public Lesson(int count, string office, string teacher, string subject, string time)
    {
        Count = count;
        Office = office;
        Teacher = teacher;
        Subject = subject;
        Time = time;
    }
}
class Producer
{
    static void Main()
    {
        // Читаем параметры подключения из переменных окружения
        var host = Environment.GetEnvironmentVariable("RABBITMQ_HOST") ?? "localhost";
        var user = Environment.GetEnvironmentVariable("RABBITMQ_USER") ?? "guest";
        var pass = Environment.GetEnvironmentVariable("RABBITMQ_PASSWORD") ?? "guest";
        // Расписание
        var lessons = new List<Lesson>
        {
            new Lesson(1, "101", "Иванов И.И.", "Математика", "08:00"),
            new Lesson(2, "202", "Петров П.П.", "Физика", "09:30"),
            new Lesson(3, "303", "Сидоров С.С.", "История", "11:00"),
            new Lesson(4, "404", "Кузнецов К.К.", "Программирование", "13:00"),
            new Lesson(5, "404", "Кузнецов К.К.", "Программирование", "17:01"),
            new Lesson(6, "505", "Смирнова А.А.", "Английский", "22:30")
        };
        var delays = new List<int> {8, 9, 11, 13, 14, 16};
        // Создаем подключение к RabbitMQ
        var factory = new ConnectionFactory() { HostName = host, UserName = user, Password = pass };
        using (var connection = factory.CreateConnection())
        using (var channel = connection.CreateModel()) {
            // Создаем очередь с именем "lessons"
            channel.QueueDeclare("lessons", false, false, false, null);
            var timeout = 0;
            foreach (var lesson in lessons)
            {
                timeout = (timeout + 1) % delays.Count;
                Thread.Sleep(1000 * 60 * delays[timeout]);
                string message = JsonSerializer.Serialize(lesson);
                var body = Encoding.UTF8.GetBytes(message);
                channel.BasicPublish(
                    exchange: "", 
                    routingKey: "lessons", 
                    basicProperties: null,
                    body: body
                );
            }
        }
    }
}
