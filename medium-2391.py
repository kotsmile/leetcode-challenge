class Solution:
    def garbageCollection(self, garbage, travel):
        total = 0
        for gargage_type in ["M", "P", "G"]:
            travel_time = 0
            new_travel_time = 0
            for house_number in range(len(garbage)):
                if gargage_type in garbage[house_number]:
                    travel_time += garbage[house_number].count(gargage_type)
                    travel_time += new_travel_time
                    new_travel_time = 0

                if house_number < len(travel):
                    new_travel_time += travel[house_number]

            total += travel_time
        return total


def main() -> None:
    print(Solution().garbageCollection(["MMM", "PGM", "GP"], [3, 10]))
    print(Solution().garbageCollection(["G", "P", "GP", "GG"], [2, 4, 3]))


if __name__ == "__main__":
    main()
