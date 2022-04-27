
LETTERS = 'abcdefghijklmnopqrstuvwxyz'

def count_letters(words: list[str]) -> dict[str, int]:
    '''count frequency of letters in list of words'''
    counts = {c:0 for c in LETTERS}
    total = 0
    for word in words:
        # remove duplicate characters in word by converting to set
        for char in set(word):
            counts[char] += 1
        total += 1
    return {k: (v/total) for k,v in counts.items()}

def load_words(filename: str) -> list[str]:
    '''load words from a file into a list.'''
    with open(filename, 'r') as f:
        lines = f.readlines()
    return[line.strip() for line in lines if line.strip()]

def main():
    words = load_words('solutions.txt')
    counts = count_letters(words)
    for k,v in sorted(counts.items(), key=lambda item: -item[1]):
        print(f'{k}: {v*100:5.2f}%')

main()
