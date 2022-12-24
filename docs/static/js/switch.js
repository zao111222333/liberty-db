const mainElement = document.getElementsByTagName('main')[0];

var linkElementList = document.getElementsByName('reference_link');
var linkList = [];
let iframeElement = document.createElement('iframe');
let iframeNav = document.createElement('nav');
iframeNav.style.position = "sticky";
iframeNav.style.height = "100vh";
iframeNav.style.top = "0";
iframeNav.style.width = "615px";
iframeNav.style.minWidth = "615px";

iframeElement.style.width = "100%";
iframeElement.style.height = "100%";
iframeElement.style.marginRight = "0px";

var activeLinkIndex = 0;
mainElement.after(iframeNav);
iframeNav.appendChild(iframeElement);

for (let index = 0; index < linkElementList.length; index++) {
    linkList.push(linkElementList[index].href);
    linkElementList[index].href = "javascript:;";
    linkElementList[index].setAttribute("onclick","swich_("+index+")");
    var content = linkElementList[index].parentElement.cloneNode(true)
    while (content.children[0]) {
        content.removeChild(content.children[0]);
    }
    if ((content.textContent==''||content.textContent==' '||content.textContent=='\n')
        &&(linkElementList[index].parentElement.previousElementSibling!=null)
        &&(linkElementList[index].previousElementSibling==null)){
        linkElementList[index].innerHTML = "Reveal in "+linkElementList[index].innerHTML;
    }else{
        linkElementList[index].innerHTML = "<br>Reveal in "+linkElementList[index].innerHTML;
    }
    linkElementList[index].style.textDecoration = "underline";
}

if (linkElementList.length!=0){
    swich_(activeLinkIndex);
}else{
    iframeElement.src = "https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html"
}

function swich_(index){
    iframeElement.src = linkList[index];
    linkElementList[activeLinkIndex].style.color = "var(--link-color)";
    activeLinkIndex = index;
    linkElementList[activeLinkIndex].style.color = "var(--type-link-color)";
}