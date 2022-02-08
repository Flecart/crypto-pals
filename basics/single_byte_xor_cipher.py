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
table = None

def score_by_simple_euristics():
    """This is just simple euristics to generate the frequency analisis stuff"""
    english = "EARIOTNSLCUDPMGHBFYWKVXZJQ"
    t = {}
    # E = 26, A = 25 etc...
    for i in range(26):
        t[english[25 - i]] = i
        t[english[25 - i].lower()] = i
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
        # lowercase
        "e":	11.607,		"m":	3.0129,	
        "a":	8.4966,		"h":	3.0034,	
        "r":	7.5809,		"g":	2.4705,	
        "i":	7.5448,		"b":	2.0720,	
        "o":	7.1635,		"f":	1.8121,	
        "t":	6.9509,		"y":	1.7779,	
        "n":	6.6544,		"w":	1.2899,	
        "s":	5.7351,		"k":	1.1016,	
        "l":	5.4893,		"v":	1.0074,	
        "c":	4.5388,		"x":	0.2902,	
        "u":	3.6308,		"z":	0.2722,	
        "d":	3.3844,		"j":	0.1965,	
        "p":	3.1671,		"q":	0.1962,	
    }
    return t

def generate_scoring_table():
    global table
    table = score_by_simple_euristics()
    print(table)
    return table 


def get_score(inp: str):
    # inp is not hex! is valid plaintext stuff here
    score_table = table if table else generate_scoring_table()
    final_score = 0
    for ch in inp:
        if ch in score_table:
            final_score += score_table[ch]
    return final_score

def frequency_attack(inp):
    results = [] # index for key to try, and value ok?
    for i in range(256):
        try:
            message = bytes.fromhex(decrypt(inp, i)).decode()
            results.append(get_score(message))
        except UnicodeDecodeError:
            results.append(0)
            continue
    
    highest_score = max(results)
    index = 0
    for k in results: # could be multiple with same score
        if k == highest_score and highest_score > 0:
            print(bytes.fromhex(decrypt(inp, key=index)))
        index += 1

frequency_attack(inp)
    