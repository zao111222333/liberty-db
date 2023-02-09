try{
pdf2htmlEX.defaultViewer = new pdf2htmlEX.Viewer({});
}catch(e){}

const queryOpts = { name: 'clipboard-write', allowWithoutGesture: false };
// const permissionStatus = await navigator.permissions.query(queryOpts);
const permissionStatus = navigator.permissions.query(queryOpts);

const color = 'rgba(227,238,0,0.2)';
const selectColor = 'rgba(180,0,170,0.15)';
var pagePosition=0;
var isScrolling;
var elementArrayList = [];
var pagePositionList = [];
var pageNum = 1;
var positionIndex = 0;
var page;
var container;
var srollBtn;
var pageInput;

container = document.getElementById('page-container');
container.onscroll = (_) => {
    window.clearTimeout( isScrolling );
    isScrolling = setTimeout(function() {
        updatePageNum();
    }, 50);
};
updatePagePosition();
var bgDiv = document.createElement("div");
bgDiv.style.position = "fixed";
bgDiv.style.zIndex = "1";
bgDiv.style.top = "0px";
bgDiv.style.left = "0px";
bgDiv.style.width = "100%";
bgDiv.style.paddingBottom = '3px';
bgDiv.style.paddingTop = '2px';
bgDiv.style.display = "flex";
bgDiv.style.justifyContent = "center";
bgDiv.style.backgroundColor = 'rgb(32,32,32)';
var Div = document.createElement("div");
Div.style.display = "flex";
Div.style.justifyContent = "space-between";
Div.className = "w0";
Div.style.flexDirection = "row-reverse";
if (document.getElementById("outline").getElementsByTagName('ul').length!=0){
    var toggleSidebarBtn = document.createElement("button");
    toggleSidebarBtn.innerHTML = "Show Menu";
    toggleSidebarBtn.style.cursor = 'pointer';
    toggleSidebarBtn.style.color = '#ffffff';
    toggleSidebarBtn.style.backgroundColor = 'rgb(63,63,63)';
    toggleSidebarBtn.addEventListener("click", function(){
        if (document.getElementById("sidebar").classList.toggle("opened")){
            toggleSidebarBtn.innerHTML = "Hide Menu";
        }else{
            toggleSidebarBtn.innerHTML = "Show Menu";
        }
    });
    Div.appendChild(toggleSidebarBtn);
}
var observer = new MutationObserver(function(mutations) {
    updatePagePosition();
    let isElement = function (o){
        return (
          typeof HTMLElement === "object" ? o instanceof HTMLElement : //DOM2
          o && typeof o === "object" && o !== null && o.nodeType === 1 && typeof o.nodeName==="string"
      );
    }
    if (isElement(container.firstElementChild)){
        let style = getComputedStyle(container.firstElementChild);
        if (style!=null){
            Div.style.width=style.width;
        }
    }
});
observer.observe(container.firstElementChild, { attributes : true, attributeFilter : ['style'] });


container.firstElementChild.appendChild(bgDiv);
bgDiv.appendChild(Div);
container.firstElementChild.style.marginTop = '24px';
var pageDiv = document.createElement("div");
pageDiv.style.flexDirection = "row";
// pageDiv.style.backgroundColor = "white";
pageDiv.style.display = "flex";
Div.appendChild(pageDiv);
pageInput = document.createElement("input");
pageInput.type = "text";
pageInput.style.width = "30px";
pageInput.style.color = '#ffffff';
pageInput.style.borderColor = 'rgb(180,180,180)';
pageInput.style.borderWidth = 'thin';
pageInput.style.backgroundColor = 'rgb(63,63,63)';
pageInput.style.zIndex = "3";
pageInput.style.marginRight = "-5px";
pageInput.style.textAlign = "right";
pageInput.setAttribute("onkeydown", "toPage(this)");
pageDiv.appendChild(pageInput);
var pageText = document.createElement("input");
pageText.style.width = "40px";
pageText.style.color = '#ffffff';
pageText.style.border = 'none';
pageText.style.background = 'none';
pageText.type = "text";
pageText.disabled = true;
pageText.value = "  / "+pagePositionList.length;
pageDiv.appendChild(pageText);
srollBtn = document.createElement("button");
srollBtn.addEventListener("click", scrollWin);
srollBtn.style.cursor = 'pointer';
srollBtn.style.color = '#ffffff';
srollBtn.style.backgroundColor = 'rgb(63,63,63)';
var btnDiv = document.createElement("div");
btnDiv.appendChild(srollBtn);
Div.appendChild(btnDiv);
window.onmessage = function(e) {
    queryString=e.data;
    updateQuery(queryString);
    addLinkBtn(queryString);
};
window.onload = function(){
    updateQuery(window.location.search);
} 


function addLinkBtn(queryString) {
    var linkBtn = document.createElement("button");
    linkBtn.innerHTML = `<a>Copy<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-link-45deg" viewBox="0 0 16 16" data-darkreader-inline-fill="" style="--darkreader-inline-fill:currentColor;">
    <path d="M4.715 6.542 3.343 7.914a3 3 0 1 0 4.243 4.243l1.828-1.829A3 3 0 0 0 8.586 5.5L8 6.086a1.002 1.002 0 0 0-.154.199 2 2 0 0 1 .861 3.337L6.88 11.45a2 2 0 1 1-2.83-2.83l.793-.792a4.018 4.018 0 0 1-.128-1.287z"></path>
    <path d="M6.586 4.672A3 3 0 0 0 7.414 9.5l.775-.776a2 2 0 0 1-.896-3.346L9.12 3.55a2 2 0 1 1 2.83 2.83l-.793.792c.112.42.155.855.128 1.287l1.372-1.372a3 3 0 1 0-4.243-4.243L6.586 4.672z"></path>
    </svg>&nbsp;&nbsp;&nbsp;</a>`;
    linkBtn.style.cursor = 'pointer';
    linkBtn.style.color = '#ffffff';
    linkBtn.style.backgroundColor = 'rgb(63,63,63)';
    linkBtn.addEventListener("click",()=>{
        let linkUrl = new URL(window.location.href);
        linkUrl.search = queryString;
        navigator.clipboard.writeText(linkUrl.toString());
    })
    btnDiv.appendChild(linkBtn);
}
function updateQuery(queryString) {
    disHighlight(elementArrayList);
    elementArrayList = [];
    let params = new URLSearchParams(queryString);
    let field = params.get('field');
    let bgnParams = params.get('bgn');
    let endParams = params.get('end');
    if (bgnParams != null && endParams != null){
        let bgnList = bgnParams.split(' ');
        let endList = endParams.split(' ');
        if (bgnList.length==endList.length){
            for (let index = 0; index < bgnList.length; index++) {
                const bgn = bgnList[index];
                const end = endList[index];
                let element = document.getElementById(bgn);
                var elementArray = [];
                highlight(element,end,elementArray);
                elementArrayList.push(elementArray);
            }
            positionIndex = elementArrayList.length-1;
        }else{
            console.error("length of Begin and Ended are NOT equal")
        }
    }
    scrollWin();
    updatePageNum();
}



function scrollWin() {
    if (elementArrayList.length==0){
        srollBtn.innerHTML = 'None';
        return
    }
    for (let index = 0; index < elementArrayList[positionIndex].length; index++) {
        const element = elementArrayList[positionIndex][index];
        element.style.backgroundColor = color;
    }
    if (positionIndex == elementArrayList.length-1){
        positionIndex = 0;
    }else{
        positionIndex = positionIndex + 1;
    }
    for (let index = 0; index < elementArrayList[positionIndex].length; index++) {
        const element = elementArrayList[positionIndex][index];
        element.style.backgroundColor = selectColor;
    }
    srollBtn.innerHTML = (positionIndex+1)+" of "+elementArrayList.length;
    const element = elementArrayList[positionIndex][0];
    var newPagePosition = element.parentElement.parentElement.offsetTop;
    var elementPosition = element.offsetTop;
    if (Math.abs(newPagePosition-pagePosition)<1000){
        container.scrollTo({top: elementPosition+newPagePosition-10, behavior: 'smooth'});
    }else{
        var newPagePositionTo;
        if (newPagePosition<pagePosition){
            newPagePositionTo = element.parentElement.parentElement.nextElementSibling.offsetTop;
        }else{
            newPagePositionTo = element.parentElement.parentElement.offsetTop;
        }
        container.scrollTo({top: newPagePositionTo});
        (function waitForLoad(index) {
            setTimeout(function() {
                elementPosition = element.offsetTop;
                if (elementPosition != 0){
                    container.scrollTo({top: elementPosition+newPagePosition-10, behavior: 'smooth'});
                }else{
                    if (--index) waitForLoad(index);
                }            
            }, 40);
        })(10);
    }
}
function disHighlight(elementArrayList) {
    for (let idx1 = 0; idx1 < elementArrayList.length; idx1++) {
        let elementArray = elementArrayList[idx1];
        for (let idx2 = 0; idx2 < elementArray.length; idx2++) {
            elementArray[idx2].style.backgroundColor = '';
        }
    }
}
function highlight(element, end, elementArray) {
    if (element!=null){
        if (element.className.substring(0,2)=='t '){
            element.style.backgroundColor = color;
            elementArray.push(element);
        }
        if (element.id!=end){
            let next = element.nextElementSibling;
            if (next==null){
                next = element.parentElement.parentElement.nextElementSibling.firstElementChild.firstElementChild;
            }
            highlight(next,end,elementArray);
        }
    }
}
function updatePageNum(){
    const offset = container.scrollTop;
    pagePosition = offset;
    var num = pageNum;
    while(offset <= pagePositionList[num-1]){
        num=num-1;
    }
    while(offset >= pagePositionList[num]){
        num=num+1;
    }
    if (num<=0){
        num = 1;
    }
    if (num>pagePositionList.length){
        num=pagePositionList.length
    }
    pageNum=num;
    pageInput.value = pageNum;
}
function toPage(element) {
    if(event.key === 'Enter') {
        var newPagePosition = pagePositionList[+(element.value)-1];
        container.scrollTop = newPagePosition;
    }
}
function updatePagePosition(){
    pagePositionList=[];
    page = container.firstElementChild;
    pagePositionList.push(page.offsetTop);
    while (page.nextElementSibling!=null) {
        page = page.nextElementSibling;
        pagePositionList.push(page.offsetTop);
    }
}