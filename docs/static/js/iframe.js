window.onload=function(){
    var switchFuncList = [];
    var activeLinkIndex = 0;
    var linkElementList = [];
    let iframeElement=document.getElementById("pdf_iframe");
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
        const BORDER_SIZE = 5;
        const BORDER_DEF_COLOR = "#2b2b2b"
        const BORDER_ACT_COLOR = "#377dce"
        const main = document.getElementsByTagName('main')[0];
        iframeElement = document.createElement('iframe');
        let iframeNav = document.createElement('nav');
        iframeNav.style.position = "sticky";
        navHight = getComputedStyle(document.getElementsByClassName("sidebar")[0]).top;
        iframeNav.style.height = "calc(100vh - "+navHight+")";
        iframeNav.style.top = navHight;
        iframeNav.style.width = "615px";
        iframeNav.style.minWidth = "300px";
        iframeNav.style.paddingLeft = BORDER_SIZE+"px";
        iframeNav.style.cursor='ew-resize';
        iframeNav.style.backgroundColor = BORDER_DEF_COLOR;
        iframeElement.id = "pdf_iframe";
        iframeElement.style.width = "100%";
        iframeElement.style.height = "100%";
        iframeElement.style.marginRight = "0px";
        main.after(iframeNav);
        iframeNav.appendChild(iframeElement);

        // resize
        let m_pos;
        
        function resize(e){
            const dx = m_pos - e.x;
            m_pos = e.x;
            iframeNav.style.width = (parseInt(getComputedStyle(iframeNav, '').width) + dx) + "px";
            main.style.width = (parseInt(getComputedStyle(main, '').width) - dx) + "px";
        }
        
        iframeNav.addEventListener("mousedown", function(e){
            iframeElement.style.pointerEvents='none';
            document.body.style.cursor='ew-resize';
            iframeNav.style.backgroundColor = BORDER_ACT_COLOR;
            if (e.offsetX < BORDER_SIZE) {
              m_pos = e.x;
              document.addEventListener("mousemove", resize, false);
            }
          }, false);
          
          document.addEventListener("mouseup", function(){
            iframeElement.style.pointerEvents='auto';
            document.body.style.cursor='';
            iframeNav.style.backgroundColor = BORDER_DEF_COLOR;
            document.removeEventListener("mousemove", resize, false);
          }, false);
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