# crack-the-bee
This program helps the user to create the data for a game like the New York Times Spelling Bee.

It requests a set of letters which the words will contain. 

The first letter must be in every word.

It reads a list of words from a file, and then filters them to match most of the game rules. So this file should have a big list of enlgish words. A big list of words can be obtained, if using a debian based linux os, with the package "wamerican-insane", which then will be stored in <b>/usr/share/dict/american-english-insane</b> 


    # On debian, e.g. Ubuntu
    sudo apt update
    sudo apt install wamerican-insane

You can use it, to create your own spelling bee game (maybe an analog version of it, and play with friends). 

Or of course, to crack the real game (achieve Genius), which is of course a way of fooling yourself/cheating.
