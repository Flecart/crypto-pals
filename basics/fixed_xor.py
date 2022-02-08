a = "1c0111001f010100061a024b53535009181c"
b = "686974207468652062756c6c277320657965"
target = "746865206b696420646f6e277420706c6179"

def xor(hex1, hex2):
    first = bytes.fromhex(hex1)
    second = bytes.fromhex(hex2)

    return bytes([x ^ y for x, y in zip(first, second)]).hex()

assert(xor(a, b) == target)