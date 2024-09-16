#!/usr/bin/env python

import os
import requests

from urllib.request import urlopen
from html.parser import HTMLParser
from html.entities import name2codepoint

scrap_url="https://lists.gnu.org/archive/mbox/emacs-devel/"
dest_dir="/home/exec/Documents/MailLists/gnu/emacs-devel/"

class MyHTMLParser(HTMLParser):
    def handle_starttag(self, tag, attrs):
        if tag == 'a':
            assert len(attrs) == 1
            attr = attrs[0]
            if len(attr) == 2 and attr[0] == 'href':

                # check if attr[1] should look like "2023-10", "2022-01"
                if len(attr[1]) != 7:
                    return
                if attr[1][4] != '-':
                    return
                
                filename=attr[1]
                # check if filename exist in dest_dir
                if os.path.isfile(dest_dir + filename):
                    print("file %s already exist" % filename)
                else:
                    # download file from scrap_url, append filename to the url
                    print("downloading %s" % filename)
                    url = scrap_url + filename
                    response = requests.get(url).content
                    content = response.decode('latin-1')

                    with open(dest_dir + filename, 'w') as f:
                        f.write(content)
                    print("done downloading %s" % filename)



def main():
    print("visiting %s", scrap_url)
    response = urlopen(scrap_url)
    html_response = response.read()
    encoding = response.headers.get_content_charset('utf-8')
    decoded_html = html_response.decode(encoding)

    parser = MyHTMLParser()
    parser.feed(decoded_html);
    

if __name__ == '__main__':
    main()
