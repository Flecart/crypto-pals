from single_byte_xor_cipher import frequency_attack 

# get the data from https://cryptopals.com/sets/1/challenges/4 page
with open("4_chal.data", "r") as f:
    line = f.readline().strip()
    counter = 0
    while line:
        print(counter, line)
        frequency_attack(line)
        line = f.readline().strip()
        counter += 1

# i get
# b'Now that the party is jumping\n'
# from this attack