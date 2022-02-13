# Wont finish this, because i have done something similiar before

from Crypto.Cipher import AES

def get_size(cipher):
    

from detection_oracle import get_random_bytes
cipher = AES.new(get_random_bytes(), AES.MODE_ECB)

def parser(input):
    entries = input.split("&")
    result = dict()

    for entry in entries:
        splitted = entry.split("=")
        if (len(splitted) != 2):
            raise 
        key, value = splitted 
        result[key] = value 
    return result 


def profile_for(email):
    if "&" in email or "=" in email:
        raise 

    return "email=" + email + "&uid=10&role=user"

def main():
    hello = profile_for("me@hello")
    print(hello)
    print(parser(hello))

main()