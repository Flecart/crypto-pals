DEBUG = None

def get_file():
    from base64 import b64decode
    with open("6_chal.data", "r") as f:
        data = f.read().strip()
    return b64decode(data) 


def hamming_distance(first, second):
    assert(len(first) == len(second))

    distance = 0
    f = lambda i: bin(i)[2:].zfill(8) # get binary form of character
    for i in range(len(first)):
        bin1 = f(first[i])
        bin2 = f(second[i])
        for i in range(8):
            if (bin1[i] != bin2[i]): distance += 1
    return distance 


def test_hamming():
    a = b"this is a test"
    b = b"wokka wokka!!!"
    assert(hamming_distance(a, b) == 37)
    print(hamming_distance(a, b))


def key_size_value(ciphertext, keysize: int, bytes_checked = 1):
    assert(keysize > 0 and bytes_checked > 0)
    # distance between first and second keysize of bytes
    distance = 0
    n_blocks = int(len(ciphertext) / keysize)
    for i in range(n_blocks):
        distance += hamming_distance(ciphertext[i : i + keysize], ciphertext[i + keysize: i + 2*keysize])
    return distance / (n_blocks * keysize)


def find_key_size(ciphertext, n_keep: int):
    results = {} 

    for i in range(2, 40):
        results[i] = key_size_value(ciphertext, i)

    if DEBUG: print("found keys, with scores: ", [x for x in sorted(results.items(), key = lambda item: item[1])][:n_keep * 2])

    return [x[0] for x in sorted(results.items(), key = lambda item: item[1])][:n_keep]


def break_ciphertext(ciphertext, keysize: int):
    """
    returns array of broken ciphertext with some keysize
    """
    assert(keysize > 0)
    breaked = [b''] * keysize
    for i in range(0, len(ciphertext), keysize):
        for j in range(keysize):
            if (len(ciphertext) > i + j): # prevent overflow
                breaked[j] += bytes([ciphertext[i + j]])
    return breaked 

def test_break_ciphertext():
    a = b"abcde" *4
    fives = break_ciphertext(a, 5)
    assert(fives == [b"a"*4, b"b"*4, b"c"*4,b"d"*4,b"e"*4])
    a = b"abc" * 4
    threes = break_ciphertext(a, 3)
    assert(threes == [b"a"*4, b"b"*4, b"c"*4])
    a = b"abcd" * 4
    twoos = break_ciphertext(a, 2)
    assert(twoos == [b"ac"*4, b"bd"*4])


def decrypt_vigenere(ciphertext, key):
    out = b''
    index = 0
    for ch in ciphertext:
        out += bytes([ch ^ key[index]])
        index = (index + 1) % len(key)

    return out


def attack_vigenere():
    from single_byte_xor_cipher import frequency_attack
    ciphertext = get_file()
    key_sizes = find_key_size(ciphertext, 1) 
    for keysize in key_sizes:
        print("Currently trying to break the cipher with keysize:", keysize)
        print("##########################################################")
        breaked_ciph = break_ciphertext(ciphertext, keysize)
        key = b''

        for cipher in breaked_ciph:
            key += bytes(frequency_attack(cipher.hex(), 1))

        print("Key found is ", key)

        try:
            print(decrypt_vigenere(ciphertext, key).decode())
        except:
            print(decrypt_vigenere(ciphertext, key))


def main():
    attack_vigenere()

if __name__ == "__main__":
    main()