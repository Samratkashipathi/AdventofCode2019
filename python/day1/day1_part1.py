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
    with open('input.txt.txt', 'r') as file:
        data = file.readlines()

    fuel = [calculate_fuel(int(mass.strip())) for mass in data]

    print(sum(fuel))


if __name__ == '__main__':
    main()
