window.onload=function(){
    var switchFuncList = [];
    var activeLinkIndex = 0;
    var linkElementList = [];
    let iframeElement=document.getElementById("pdf_iframe_module");
    var linkList = [];
    Object.defineProperty(this, 'linkIndexWatch', {
        get: function () { return activeLinkIndex; },
        set: function (v) {
          linkElementList[activeLinkIndex].style.color = "var(--link-color)";
          activeLinkIndex = v;
          linkElementList[activeLinkIndex].style.color = "var(--type-link-color)";
          iframeElement.contentWindow.postMessage(linkList[activeLinkIndex][1],'*');
          iframeElement.addEventListener("load", function() {
            iframeElement.contentWindow.postMessage(linkList[activeLinkIndex][1],'*');
          });
        }
      });
    if (iframeElement==null){
        iframeElement = document.createElement('iframe');
        let iframeNav = document.createElement('nav');
        iframeNav.style.position = "sticky";
        iframeNav.style.height = "100vh";
        iframeNav.style.top = "0";
        iframeNav.style.width = "615px";
        iframeNav.style.minWidth = "615px";
        iframeElement.id = "pdf_iframe_module";
        iframeElement.style.width = "100%";
        iframeElement.style.height = "100%";
        iframeElement.style.marginRight = "0px";
        document.getElementsByTagName('main')[0].after(iframeNav);
        iframeNav.appendChild(iframeElement);
    }
    linkElementList=document.getElementsByName('reference_link');
    for (let index = 0; index < linkElementList.length; index++) {
        let linkUrl = new URL(linkElementList[index].href);
        let linkSearch = linkUrl.search;
        linkUrl.search='';
        linkList.push([linkUrl.toString(),linkSearch]);
        linkElementList[index].removeAttribute("href");
        linkElementList[index].style.cursor = "pointer";
        switchFuncList.push(function(){
            linkIndexWatch = index;
            if (iframeElement.src != linkList[index][0]){
                iframeElement.src = linkList[index][0];
            }
        });
        linkElementList[index].addEventListener("click", switchFuncList[index]);
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
    if (switchFuncList.length!=0){
        iframeElement.src = linkList[0][0];
        switchFuncList[0]();
    }else{
        iframeElement.src = "https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html"
    }
}