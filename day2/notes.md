How do we know where should we split first?
- The split starts at the length of the value, for example, if its 12 digits long:

212121211812 (12 / 12)
212121 | 211812 (12 / 6)
2121 | 2121 | 1812 (12 / 4)
212 | 121 | 211 | 812 (12 / 3)
21 | 21 | 21 | 21 | 18 | 12 (12 / 2)

------------------------------------------------

212121211812 
0..11 -> 0 to length
splits at 6 (check in checks)
word_one = [0..5] 
word_two = [6..11]
