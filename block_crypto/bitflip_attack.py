from pydoc import plain
from traceback import clear_frames
from Crypto.Cipher import AES 
from detection_oracle import get_random_bytes

key = get_random_bytes()
IV = get_random_bytes()

def pad(text, size = 16):
    lenght = len(text)
    n_pad = size - (lenght % size)
    return text + bytes([n_pad]) * n_pad 

def get_ciphertext(text):
    prepend = b"comment1=cooking%20MCs;userdata="
    append = b";comment2=%20like%20a%20pound%20of%20bacon"
    cleared_text = b""
    for char in text:
        if char == b';' or char == b'=':
            cleared_text += f'"{char}"'.encode()
        else:
            cleared_text += bytes([char])
    chained_text = prepend + cleared_text + append
    padded_text = pad(chained_text)

    cipher = AES.new(key, AES.MODE_CBC, IV)

    return cipher.encrypt(padded_text)

def is_admin(ciphertext):
    cipher = AES.new(key, AES.MODE_CBC, IV)
    plaintext = cipher.decrypt(ciphertext)
    entries = plaintext.split(b";")
    result = dict()

    for entry in entries:
        splitted = entry.split(b"=")
        if (len(splitted) != 2):
            print(splitted)
            raise 
        k, value = splitted 
        result[k] = value 

    print(result)
    if b"admin" in result:
        return True 
    else:
        return False 

def flip_attack():
    wanted_plaintext = pad(b"a;admin=true")
    xor = lambda a,b: bytes([x^y for x,y in zip(a,b)])
    first = b"a"
    ciphertext = get_ciphertext(first)
    second_block = ciphertext[16:32]
    crafted_block = xor(second_block, wanted_plaintext)

    new_ciph = get_ciphertext(crafted_block)

    print(is_admin(b"".join([new_ciph[:16], b"\x00"* 16, new_ciph[32:]])))

def main():
    ciph = get_ciphertext(b";admin") 
    print(is_admin(ciph))


flip_attack()