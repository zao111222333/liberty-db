window.onload=function(){
    var mouseDown = false;
    document.body.onmousedown = function() { 
        mouseDown = true;
    }
    document.body.onmouseup = function() {
        mouseDown = false;
    }
    let isMobile = /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(navigator.userAgent);
    
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
        const BORDER_SIZE = 4;
        const BORDER_DEF_COLOR = 'rgba(55,125,206,0)';
        const BORDER_ACT_COLOR = 'rgb(55,125,206)';
        const main = document.getElementsByTagName('main')[0];
        main.style.minWidth = "400px";
        iframeElement = document.createElement('iframe');
        let iframeNav = document.createElement('nav');
        iframeNav.style.display="flex";
        if (isMobile){
            iframeNav.style.position = "fixed";
            iframeNav.style.height = "40vh";
            iframeNav.style.bottom = '0';
            iframeNav.style.width = "100vw";
            main.style.paddingBottom = '42vh';
        }else{
            let navHight = getComputedStyle(document.getElementsByClassName("sidebar")[0]).top;
            iframeNav.style.position = "sticky";
            iframeNav.style.height = "calc(100vh - "+navHight+")";
            iframeNav.style.top = navHight;
            iframeNav.style.width = "615px";
            iframeNav.style.minWidth = "300px";
            iframeNav.style.flexDirection="row";
        }
        iframeElement.id = "pdf_iframe";
        iframeElement.style.width = "100%";
        iframeElement.style.height = "100%";
        iframeElement.style.border = 'none';
        iframeElement.style.marginRight = "0px";
        iframeElement.setAttribute('allow', 'clipboard-write');
        let resizeBar = document.createElement('div');
        resizeBar.style.height = "100%";
        resizeBar.style.marginLeft = -BORDER_SIZE+"px";
        resizeBar.style.marginRight = -BORDER_SIZE+"px";
        resizeBar.style.backgroundColor = BORDER_DEF_COLOR;
        resizeBar.style.width = 2*BORDER_SIZE+"px";
        resizeBar.style.minWidth = 2*BORDER_SIZE+"px";
        resizeBar.style.zIndex = '100';
        resizeBar.style.cursor='col-resize';
        main.after(iframeNav);
        iframeNav.appendChild(resizeBar);
        iframeNav.appendChild(iframeElement);

        // resize
        let m_pos;
        
        function resize(e){
            const dx = m_pos - e.x;
            m_pos = e.x;
            if (dx>0){
                main.style.width = (parseInt(getComputedStyle(main, '').width) - dx) + "px";
            }
            iframeNav.style.width = (parseInt(getComputedStyle(iframeNav, '').width) + dx) + "px";
            let mainStyle = getComputedStyle(main, '')
            let iframeStyle = getComputedStyle(iframeNav, '')
            let seted = false;
            if (!seted&&(parseInt(mainStyle.width)<=parseInt(mainStyle.minWidth))){
                document.body.style.cursor='e-resize';
                seted = true;
            }
            if (!seted&&(parseInt(iframeStyle.width)<=parseInt(iframeStyle.minWidth))){
                document.body.style.cursor='w-resize';
                seted = true;
            }
            if (!seted){
                document.body.style.cursor='col-resize';
            }
        }
        
        resizeBar.addEventListener('mouseover', (event) => {
            resizeBar.style.backgroundColor = BORDER_ACT_COLOR;
            resizeBar.style.width =  2*BORDER_SIZE+"px";
        });
        resizeBar.addEventListener('mouseleave', (event) => {
            if (!mouseDown){
                resizeBar.style.backgroundColor = BORDER_DEF_COLOR;
                resizeBar.style.width = BORDER_SIZE+"px";
            }
        });


        resizeBar.addEventListener("mousedown", function(e){
            iframeElement.style.pointerEvents='none';
            main.style.userSelect="none";
            m_pos = e.x;
            document.addEventListener("mousemove", resize, false);
          }, false);
          
          document.addEventListener("mouseup", function(){
            resizeBar.style.width = BORDER_SIZE+"px";
            iframeElement.style.pointerEvents='auto';
            document.body.style.cursor='';
            main.style.userSelect="";
            resizeBar.style.backgroundColor = BORDER_DEF_COLOR;
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