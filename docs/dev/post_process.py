#!/usr/bin/env python3

import sys
import os
from bs4 import BeautifulSoup
file_name=str(sys.argv[1])

# Add id
with open(file_name, 'r') as read_file:
    html_source = read_file.read()

soup = BeautifulSoup(html_source, 'html.parser')
container=soup.find(id="page-container")
pages = container.findChildren("div" , recursive=False)
for page_num, page in enumerate(pages):
    items = page.findChildren("div" , recursive=False)[0].findChildren("div" , recursive=False)
    for item_num, item in enumerate(items):
        id=str(page_num+1)+"."+str(item_num)
        item.attrs['id'] = id

# Add Highlight
js = soup.new_tag("script")
js['src']="highlight.js"
soup.body.insert(len(soup.body.contents), js)

with open(file_name, 'w') as save_file:
    save_file.write(str(soup))
    