# (in ./static) run shell: ../dev/pdf2html.sh
PDF_FILE="liberty07_03.pdf"
HTML_FILE=${PDF_FILE%.pdf*}".html"
alias pdf2htmlEX='docker run -ti --rm -v "`pwd`":/pdf -w /pdf pdf2htmlex/pdf2htmlex:0.18.8.rc2-master-20200820-ubuntu-20.04-x86_64'
pdf2htmlEX --bg-format svg pdf/liberty07_03.pdf
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
$SCRIPT_DIR/post_process.py $HTML_FILE
