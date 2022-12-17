#! /usr/bin/python3

import sys
import os
import pathlib
file_name=str(sys.argv[1])

# creating a variable and storing the text
# that we want to search
search_text = "</body>"
replace_text = "<script src=\"highlight.js\"></script></body>"
# with open(file_name, 'r') as file:

# Opening our text file in read only
# mode using the open() function
with open(file_name, 'r') as file:
    # Reading the content of the file
    # using the read() function and storing
    # them in a new variable
    data = file.read()
  
    # Searching and replacing the text
    # using the replace() function
    data = data.replace(search_text, replace_text)
  
# Opening our text file in write only
# mode to write the replaced content
with open(file_name, 'w') as file:
  
    # Writing the replaced data in our
    # text file
    file.write(data)