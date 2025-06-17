namespace work_1_testing_white_box
{
    enum UserStatus {
        Regular = 1,
        Premium = 2,
        Gold = 3,
        NONE = 4
    }

    internal class Program
    {
        static Dictionary<string, UserStatus> Users = new Dictionary<string, UserStatus>()
        {
            { "Simple", UserStatus.Gold},
            { "Alex", UserStatus.Regular},
            { "Vika", UserStatus.Premium}
        };
        static Dictionary<UserStatus, float> StatusCost = new Dictionary<UserStatus, float>()
        {
            {UserStatus.Gold, 1500.00f},
            {UserStatus.Premium, 1000.00f},
            {UserStatus.Regular, 500.00f},
            {UserStatus.NONE, 0f}
        };
        static Dictionary<UserStatus, float> SaleProcent = new Dictionary<UserStatus, float>()
        {
            {UserStatus.NONE, 10f},
            {UserStatus.Regular, 20f},
            {UserStatus.Premium, 30f},
            {UserStatus.Gold, 50f}
        };
        // Купоны
        static Dictionary<string, float> Coupons = new Dictionary<string, float>()
        {
            { "SALE10", 10f },
            { "SALE20", 20f },
            { "SUPER50", 50f }
        };
        private const float saleCost = 1200.99f;
        private const float defaltSale = 10f;
        private static void Main()
        {
            bool isExit = false;
            string? uiBuf = null;
            string userName = "";
            bool user_has_status = false;
            float globalCost = 0f;
            UserStatus status = UserStatus.NONE;
            float promoSale = 0f;
            // Ввод имени
            while (!isExit)
            {
                Console.WriteLine("Введите свое имя латинскими буквами с заглавной :");
                uiBuf = Console.ReadLine();
                if (!string.IsNullOrEmpty(uiBuf))
                {
                    userName = uiBuf;
                    isExit = true;
                }
            }
            // Ввод суммы заказа
            isExit = false;
            while (!isExit)
            {
                Console.WriteLine("Введите сумму заказа : ");
                uiBuf = Console.ReadLine();
                if (float.TryParse(uiBuf, out float orderCost))
                {
                    user_has_status = Users.ContainsKey(userName);
                    if (user_has_status)
                    {
                        status = Users[userName];
                    }
                    globalCost = orderCost;
                    isExit = true;
                }
                else
                {
                    Console.WriteLine("Вы ввели не число");
                }
            }
            // Ввод купона
            Console.WriteLine("Введите промокод (если есть) или нажмите Enter для пропуска:");
            uiBuf = Console.ReadLine();
            if (!string.IsNullOrEmpty(uiBuf))
            {
                if (Coupons.ContainsKey(uiBuf))
                {
                    promoSale = Coupons[uiBuf];
                    Console.WriteLine($"Промокод применён: дополнительная скидка {promoSale}%");
                }
                else
                {
                    Console.WriteLine("Такого промокода нет.");
                }
            }
            // Расчёт и вывод итоговой стоимости
            string sStatus = "";
            if (status == UserStatus.NONE)
            {
                sStatus = "Нет статуса";
                Console.WriteLine($"Статус : {sStatus}\n");
                if (globalCost < saleCost)
                {
                    Console.WriteLine($"Цена ниже условия скидки {saleCost}");
                }
                else
                {
                    var sale = globalCost / 100 * defaltSale;
                    globalCost -= sale;
                    Console.WriteLine($"Скидка : {sale} ({defaltSale}%)");
                }
            }
            else
            {
                switch (status)
                {
                    case UserStatus.Regular:
                        sStatus = "Регулярный";
                        break;
                    case UserStatus.Premium:
                        sStatus = "Премиальный";
                        break;
                    case UserStatus.Gold:
                        sStatus = "Золотой";
                        break;
                    case UserStatus.NONE:
                        sStatus = "Нет статуса";
                        break;
                }
                Console.WriteLine($"Статус пользователя : {sStatus}");
                bool sale_apply = globalCost > StatusCost[status];
                var sale = globalCost / 100 * SaleProcent[status];
                if (sale_apply)
                {
                    Console.WriteLine($"Скидка применена в размере : {sale} ({SaleProcent[status]}%)");
                    globalCost -= sale;
                }
                else Console.WriteLine($"Скидка применяется с {StatusCost[status]}");
            }
            // Применение промокода (если был)
            if (promoSale > 0f)
            {
                var promoDiscount = globalCost / 100 * promoSale;
                Console.WriteLine($"Дополнительная скидка по промокоду: {promoDiscount} ({promoSale}%)");
                globalCost -= promoDiscount;
            }
            Console.WriteLine($"ИТОГО : {globalCost}");
        }
    }
}
