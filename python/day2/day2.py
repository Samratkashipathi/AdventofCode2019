def calculate(input_arr) -> int:
    data = input_arr[:]

    for i in range(0, len(data), 4):
        opcode = data[i]
        if opcode == 99:
            return data[0]

        num1 = data[data[i + 1]]
        num2 = data[data[i + 2]]
        pos = data[i + 3]

        if opcode == 1:
            data[pos] = num1 + num2

        if opcode == 2:
            data[pos] = num1 * num2

    return data[0]


def main():
    with open('input.txt', 'r') as file:
        data = file.readlines()

    data = [item.split(',') for item in data]
    for item in data:
        item = [int(ele.strip()) for ele in item]
        for noun in range(100):
            for verb in range(100):
                item[1] = noun
                item[2] = verb

                result = calculate(item)
                if result == 19690720:
                    print(100 * noun + verb)


if __name__ == '__main__':
    main()
