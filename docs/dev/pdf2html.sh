# (in ./static) run shell: ../dev/pdf2html.sh
PDF_FILE="Liberty_User_Guides_and_Reference_Manual_Suite_Version_2017.06.pdf"
alias pdf2htmlEX='docker run -ti --rm -v "`pwd`":/pdf -w /pdf pdf2htmlex/pdf2htmlex:0.18.8.rc2-master-20200820-ubuntu-20.04-x86_64'
pdf2htmlEX --bg-format svg --embed-javascript 0 2007.03/user_guide.pdf       2007.03/user_guide.html
pdf2htmlEX --bg-format svg --embed-javascript 0 2007.03/_user_guide.pdf      2007.03/_user_guide.html
pdf2htmlEX --bg-format svg --embed-javascript 0 2020.09/user_guide.pdf       2020.09/user_guide.html
pdf2htmlEX --bg-format svg --embed-javascript 0 2020.09/reference_manual.pdf 2020.09/reference_manual.html
rm -rf *.js
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
$SCRIPT_DIR/post_process.py 2007.03/user_guide.html
$SCRIPT_DIR/post_process.py 2007.03/_user_guide.html
$SCRIPT_DIR/post_process.py 2020.09/user_guide.html
$SCRIPT_DIR/post_process.py 2020.09/reference_manual.html