from Crypto.Cipher import AES 

def decrypt(ciphertext, key, iv = b'\x00' * 16):
    cipher = AES.new(key, AES.MODE_ECB)
    prec = iv

    plaintext = b''
    for i in range(0, len(ciphertext), 16):
        block = ciphertext[i:i+16]
        plaintext += xor(prec, cipher.decrypt(block))
        prec = block 
    return plaintext

def xor(first, second):
    return bytes([x ^ y for x, y in zip(first, second)])


if __name__ == "__main__":
    with open("10_chal.data", "r") as f:
        data = f.read().strip()
    from base64 import b64decode
    data = b64decode(data)

    print(decrypt(data, b"YELLOW SUBMARINE").decode())