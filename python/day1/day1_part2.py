from math import floor


def calculate_fuel(mass: int) -> float:
    """
    Calculate fuel based on the mass

    Args:
        mass (int): Mass to calculate fuel

    Returns:
        None
    """
    fuel = floor(mass / 3) - 2
    return fuel


def main():
    with open('input.txt', 'r') as file:
        fuel = file.readlines()

    fuel = list(map(int, fuel))
    total_fuel = 0

    while any(item > 0 for item in fuel):
        fuel = [calculate_fuel(mass=mass) for mass in fuel]
        fuel = list(filter(lambda x: x > 0, fuel))
        total_fuel = total_fuel + sum(fuel)

    print(total_fuel)


if __name__ == '__main__':
    main()
