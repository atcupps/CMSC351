a = 'a'
b = 'b'
c = 'c'
d = 'd'
e = 'e'

letters = [a, b, c, d, e]
e_count = 0
total = 0

for first in letters:
    available1 = {a, b, c, d, e}
    available1.remove(first)
    for second in list(available1):
        available2 = {a, b, c, d, e}
        available2.remove(first)
        available2.remove(second)
        for third in list(available2):
            available3 = {a, b, c, d, e}
            available3.remove(first)
            available3.remove(second)
            available3.remove(third)
            for fourth in list(available3):
                available4 = {a, b, c, d, e}
                available4.remove(first)
                available4.remove(second)
                available4.remove(third)
                available4.remove(fourth)
                for fifth in list(available4):
                    assert(len(list(available4)) == 1)
                    total += 1
                    result = []
                    for letter in [first, second, third, fourth, fifth]:
                        if len(result) < 3:
                            if letter == a and not (b in result and e in result):
                                result += [a]
                            if letter == b and not (a in result and e in result):
                                result += [b]
                            if letter == c and not (d in result and e in result):
                                result += [c]
                            if letter == d and not (c in result and e in result):
                                result += [d]
                            if letter == e and not ((c in result and d in result) or (b in result and a in result)):
                                result += [e]
                    if e in result:
                        e_count += 1
                          
print(f"Triplets with e: {e_count}")
print(f"Total: {total}")