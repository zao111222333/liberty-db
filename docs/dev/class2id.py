#!/usr/bin/env python3

from bs4 import BeautifulSoup

import subprocess

def get_clipboard_data():
    p = subprocess.Popen(['pbpaste'], stdout=subprocess.PIPE)
    retcode = p.wait()
    data = p.stdout.read()
    return data.decode("utf-8") 

def set_clipboard_data(data):
    data = bytes(data, "utf-8")
    p = subprocess.Popen(['pbcopy'], stdin=subprocess.PIPE)
    p.stdin.write(data)
    p.stdin.close()
    retcode = p.wait()

if __name__ == "__main__":
    with open('/Users/junzhuo/Developer/EDA/liberty-rs/docs/static/liberty07_03.html', 'r') as read_file:
        html_source = read_file.read()
    soup = BeautifulSoup(html_source, 'html.parser')
    while True:
        # container=soup.find(id="page-container")
        print("Input ClassName:")
        # out = string.replace(" ", ".")
        # set_clipboard_data(out)
        className = str(input()).replace(".", " ")
        item = soup.find("div", {"class": className})
        out = item.get('id')
        print("Output ID: "+out)
        set_clipboard_data(out)
        