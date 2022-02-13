inp = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"

def decrypt(hex_message, key):
    message = bytes.fromhex(hex_message)
    return bytes([x ^ key for x in message]).hex()

def key_bruteforce(hex_message):
    for i in range(256):
        try:
            print(bytes.fromhex(decrypt(hex_message, i)).decode())
        except:
            continue

# key_bruteforce(hex_message=input)
# message retrieved:
# b"Cooking MC's like a pound of bacon"


# This one is just human friendly bruteforce
def interactive_decrypt(inp, n=3):
    english = "EARIOTNSLCUDPMGHBFYWKVXZJQ"
    c = Counter(bytes.fromhex(inp))
    main_bit = c.most_common(1)[0][0]

    while True:
        ch = input("What is the guessed value for the most common bit? ")
        if (len(ch) != 1):
            print("Invalid input, need to be single char")
        key = main_bit ^ ord(ch)
        print(bytes.fromhex(decrypt(inp, key)))
        if (input("Is this the right answer? [y to exit] ") == 'y'):
            break 


# THIS ONE DOWN HERE IS THE IMPLEMENTED
# FREQ ATTACK!
from collections import Counter 

def score_by_simple_euristics():
    """
    This is just simple euristics to generate the frequency analisis stuff
    It works fine, as well as entropy thing lol!
    """
    english = "EARIOTNSLCUDPMGHBFYWKVXZJQ"
    t = {}
    # E = 26, A = 25 etc...
    for i in range(26):
        t[english[25 - i]] = i
        t[english[25 - i].lower()] = i

    # some common stuff, hope it helps to change
    punteggiatura = [" ", ".", ",", "'"]
    for i in punteggiatura:
        t[i] = 10
    return t

def score_by_frequency():
    """I'm assigning the values based on the frequency of the single letters
    Giving a higher score to the most frequent ones i'm trying to facilitate
    the presence of high frequency letters (the result is similiar to the euristics
    above, no reason to use this one)

    I don't know why, but actually the other one works better than this
    """
    # https://www3.nd.edu/~busiforc/handouts/cryptography/letterfrequencies.html
    t = { 
        "E":	11.607,		"M":	3.0129,	
        "A":	8.4966,		"H":	3.0034,	
        "R":	7.5809,		"G":	2.4705,	
        "I":	7.5448,		"B":	2.0720,	
        "O":	7.1635,		"F":	1.8121,	
        "T":	6.9509,		"Y":	1.7779,	
        "N":	6.6544,		"W":	1.2899,	
        "S":	5.7351,		"K":	1.1016,	
        "L":	5.4893,		"V":	1.0074,	
        "C":	4.5388,		"X":	0.2902,	
        "U":	3.6308,		"Z":	0.2722,	
        "D":	3.3844,		"J":	0.1965,	
        "P":	3.1671,		"Q":	0.1962,
    }

    # adding some + value for punti
    punteggiatura = [" ", ".", ",", "'"]
    for i in punteggiatura:
        t[i] = 2
    return t

def generate_scoring_table(func = score_by_simple_euristics):
    return func() 

def general_get_score(plaintext: str, get_score_table, scorer):
    plaintext = plaintext.upper()

    score_table = get_score_table()
    final_score = 0
    counter = Counter(plaintext)
    total = counter.total()

    for ch in plaintext:
        if ch in score_table:
            final_score += scorer(score_table[ch], counter[ch] / total)

    return final_score


def get_entropy_score(inp: str):
    from math import log2
    return general_get_score(inp, score_by_frequency, lambda freq, prob: freq * log2(1/prob))


def get_euristics_score(inp: str):
    return general_get_score(inp, score_by_simple_euristics, lambda freq, _: freq)


def frequency_attack(inp, n_keys: int = 2, get_score = get_euristics_score):
    results = {} # index for key to try, and value ok?
    for i in range(256):
        try:
            message = bytes.fromhex(decrypt(inp, i)).decode()
            results[i] = get_score(message)
        except UnicodeDecodeError:
            continue

    print([x for x in reversed(sorted(results.items(), key=lambda item: item[1]))][:n_keys * 3])
    return [key for key, _ in reversed(sorted(results.items(), key=lambda item: item[1]))][:n_keys]


def show_results(inp, keys):
    for key in keys:
        print(bytes.fromhex(decrypt(inp, key)))

if __name__ == "__main__":
    show_results(inp, frequency_attack(inp))
    