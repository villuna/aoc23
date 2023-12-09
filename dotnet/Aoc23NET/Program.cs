using Framework;
using Fs;
using Cs;
using System.IO;

namespace Aoc23NET {
    class Aoc23 {
        static void Main(string[] args) {
            List<IDay?> days = [new Day1(), new Day2()];

            if (args.GetLength(0) != 1) {
                Console.WriteLine("usage: aoc23cs [day]");
                return;
            }

            if (!int.TryParse(args[0], out int dayNum)) {
                Console.WriteLine("\"" + args[0] + "\" is not a valid day number");
                return;
            }

            if (dayNum > 0 && dayNum <= days.Count && days[dayNum - 1] is IDay day) {
                try {
                    StreamReader reader = new StreamReader("../input/day" + dayNum + ".txt");
                    string input = reader.ReadToEnd();
                    day.Run(input);
                } catch (FileNotFoundException) {
                    Console.WriteLine("Input file does not exist. Please create it at [REPO ROOT]/input/day" + dayNum + ".txt");
                    return;
                }
            } else {
                Console.WriteLine("Day not solved");
            }
        }
    }
}
