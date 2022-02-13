def pad(text, size):
    lenght = len(text)
    n_pad = size - (lenght % size)
    return text + bytes([n_pad]) * n_pad 

if __name__ == "__main__":
    print(pad(b"YELLOW SUBMARINE", 20))
