#! /usr/bin/python3
# Read class name from clipboard, e.g. "t m0 x8 h5 y2899 ff1 fs2 fc2 sc0 ls0 ws0"
# Put processed id to your clipboard, e.g. "t.m0.x8.h5.y2899.ff1.fs2.fc2.sc0.ls0.ws0"
# Reference:
# Read from and Write to clipboards in python scripts, works on Mac at least.
# https://gist.github.com/XuankangLin/7ec82f80a0044a52330720244de2d15a

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
    string = get_clipboard_data()
    print("Get input from your clipboard: "+string)
    out = string.replace(" ", ".")
    set_clipboard_data(out)
    print("Put output into your clipboard: "+out)