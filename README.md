# crack
This program helps the user to create the data/ or solve for a games like (Spelling) Bee or Word(le)

## Requirements

The games where *crack* aids require a list of words in the language used for the game. One can either provide a file path or an url.

### File as a source
A big list of words can be obtained, if using a debian based linux os, with the package "wamerican-insane", which then will be stored in <b>/usr/share/dict/american-english-insane</b> 

    # On debian, e.g. Ubuntu
    sudo apt update
    sudo apt install wamerican-insane

So when running *crack <game>*, the option for the file can be: *--file_path /usr/share/dict/american-english-insane*

### URL as a source

when running *crack <game>*, the option for the url can be (for example): *--url https://raw.githubusercontent.com/dwyl/english-words/refs/heads/master/words.txt*

# Bee

This program accepts 7 letters to be used (the first letter being the one that is always present), and together with a word list, it dumps all the possible words that match the criteria. In simple terms, it reads a list of words from a file, and then filters them to match most of the game rules. So this file should have a big list of enlgish words. 

You can use it, to create your own spelling bee game (maybe an analog version of it, and play with friends). 

Or of course, to crack the real game (achieve Genius), which is of course a way of fooling yourself/cheating.

# Word

This is an interactive program that helps you to solve the word(le) puzzles. So you need to cycle through it while playing the game. It will definitely not solve it in 1 or 2 tries, but the chances to do it in 3 or 4 tries are high, and it can also be used as inspiration for words that match the requirements.
