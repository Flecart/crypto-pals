from Crypto.Cipher import AES

key = b"YELLOW SUBMARINE"
def get_file():
    from base64 import b64decode
    with open("7_chal.data", "r") as f:
        data = f.read().strip()
    return b64decode(data) 

def decryptAES():
    ciphertext = get_file()
    cipher = AES.new(key, AES.MODE_ECB)
    print(cipher.decrypt(ciphertext))

def chal_8():

    # i assume that a block of plaintext is repeated, so we would have 2 same ciphertexts
    def has_same(line):
        size = 16
        blocks = []
        for i in range(0, len(line), size):
            if line[i:i+size] in blocks:
                return True 
            else:
                blocks.append(line[i:i+size])
        return False 

    with open("8_chal.data", "r") as f:
        line = f.readline().strip()
        while line:
            if has_same(bytes.fromhex(line)):
                print(line)
                return 

            line = f.readline().strip()
            


if __name__ == "__main__":
    chal_8()