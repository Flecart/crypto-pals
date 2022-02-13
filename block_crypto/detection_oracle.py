from mimetypes import guess_extension
from Crypto.Cipher import AES 
from Crypto.Util.number import long_to_bytes
import secrets
import random

from pad import pad

def get_random_bytes(size = 16):
    random_bytes = long_to_bytes(secrets.randbits(size * 8))
    while len(random_bytes) < 16: # accade se ci sono molti byte a 0 all'inizio
        random_bytes = b'\x00' + random_bytes
    return random_bytes

def encryption_oracle(input):
    before = random.randrange(5, 10)
    after = random.randrange(5, 10)
    input = get_random_bytes(before) + input + get_random_bytes(after)
    input = pad(input, 16)

    ecb = AES.new(get_random_bytes(), AES.MODE_ECB)
    cbc = AES.new(get_random_bytes(), AES.MODE_CBC, get_random_bytes())
    is_ecb = random.randrange(0,2)
    if is_ecb:
        ciphertext = ecb.encrypt(input)
    else:
        ciphertext = cbc.encrypt(input)

    return ciphertext, is_ecb


def get_entropy(text):
    from collections import Counter 
    from math import log2
    c = Counter(text)
    total = 0
    for key in c:
        total += c[key]
    entropy = 0

    for byte in c:
        entropy += (1/256) * log2(total / c[byte])
    return entropy

def detector():
    # WARNING: DOESN'T WORK
    text = b"BEAUTIFUL RANDOM TEXT"
    ciphertext, is_ecb = encryption_oracle(text)
    entropy = get_entropy(ciphertext)
    guess_is_ecb = entropy > 0.7
    return guess_is_ecb == is_ecb

def detect():
    text = b"BEAUTIFUL RANDOM TEXT"
    cipher, is_ecb = encryption_oracle(text)

    chunkSize = 16
    chunks = []
    for i in range(0, len(cipher), chunkSize):
        chunks.append(cipher[i:i+chunkSize])

    uniqueChunks = set(chunks)
    if len(chunks) > len(uniqueChunks):
        return is_ecb == True

    return is_ecb == False

if __name__ == "__main__":
    guessed = 0
    for i in range(10000):
        if detector():
            guessed += 1
    print("N times guessed right: ", guessed)
    # print(encryption_oracle(b"hello"))