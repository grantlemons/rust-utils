from random import randrange

def write_randoms(num):
    with open(f"randoms/{num}randoms.txt", 'w') as file:
        for _ in range(0, num):
            file.write(str(randrange(0, 65535)) + '\n')

write_randoms(100)
write_randoms(1000)
write_randoms(10000)
write_randoms(100000)
write_randoms(1000000)
