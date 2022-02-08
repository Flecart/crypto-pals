from base64 import b64encode
string = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"

byt = bytes.fromhex(string)
print(byt)
print(b64encode(byt));
