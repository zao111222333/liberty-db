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

if (linkElementList!=null){
    iframeElement.src = linkElementList[0].href;
}else{
    iframeElement.src = "https://zao111222333.github.io/liberty-rs/liberty07_03.html"
}
mainElement.after(iframeNav);
iframeNav.appendChild(iframeElement);

for (let index = 0; index < linkElementList.length; index++) {
    linkList.push(linkElementList[index].href);
    linkElementList[index].href = "javascript:;";
    linkElementList[index].setAttribute("onclick","www("+index+")")
}

function www(index){
    iframeElement.src = linkList[index];
}