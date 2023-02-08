/*
Demo:
https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
?field=test
&bgn
=t.m0.x9.h4.y2926.ff1.fs2.fc2.sc0.ls0.ws0
+t.m0.x9.h4.y2956.ff1.fs2.fc2.sc0.ls0.ws0
+t.m0.xb.h7.y285c.ff1.fs2.fc2.sc0.ls0.ws0
&end
=t.m0.xb.h8.yf8d.ff7.fs2.fc2.sc0.ls0.ws0
+t.m0.xb.h8.y297c.ff7.fs2.fc2.sc0.ls0.ws0
+t.m0.x39.h8.y2874.ff8.fs2.fc2.sc0.ls0.ws0
*/

try{
pdf2htmlEX.defaultViewer = new pdf2htmlEX.Viewer({});
}catch(e){}

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
var btn;
var pageInput;
if (document.getElementById("outline").getElementsByTagName('ul').length!=0){
    var toggleSidebarBtn = document.createElement("button");
    toggleSidebarBtn.innerHTML = "show menu";
    toggleSidebarBtn.addEventListener("click", function(){
        if (document.getElementById("sidebar").classList.toggle("opened")){
            toggleSidebarBtn.innerHTML = "hide menu";
        }else{
            toggleSidebarBtn.innerHTML = "show menu";
        }
    });
    Div.appendChild(toggleSidebarBtn);
}
container = document.getElementById('page-container');
container.onscroll = (_) => {
    window.clearTimeout( isScrolling );
    isScrolling = setTimeout(function() {
        updatePageNum();
    }, 50);
};
page = container.firstElementChild;
pagePositionList.push(page.offsetTop);
while (page.nextElementSibling!=null) {
    page = page.nextElementSibling;
    pagePositionList.push(page.offsetTop);
}
var Div = document.createElement("div");
Div.style.display = "flex";
Div.style.position = "fixed";
Div.style.zIndex = "1";
Div.style.top = "0px";
Div.style.justifyContent = "space-between";
Div.className = "w0";
Div.style.flexDirection = "row-reverse";


container.firstElementChild.appendChild(Div);
var pageDiv = document.createElement("div");
pageDiv.style.flexDirection = "row";
pageDiv.style.backgroundColor = "white";
pageDiv.style.display = "flex";
Div.appendChild(pageDiv);
pageInput = document.createElement("input");
pageInput.type = "text";
pageInput.style.width = "30px";
pageInput.style.zIndex = "3";
pageInput.style.marginRight = "-5px";
pageInput.style.textAlign = "right";
pageInput.setAttribute("onkeydown", "toPage(this)");
pageDiv.appendChild(pageInput);
var pageText = document.createElement("input");
pageText.style.width = "40px";
pageText.type = "text";
pageText.disabled = true;
pageText.value = " / "+pagePositionList.length;
pageDiv.appendChild(pageText);

btn = document.createElement("button");
btn.addEventListener("click", scrollWin);
Div.appendChild(btn);
window.onmessage = function(e) {
    updateQuery(e.data);
};
window.onload = function(){
    updateQuery(window.location.search);
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
            scrollWin();
        }else{
            console.error("length of Begin and Ended are NOT equal")
        }
    }
    updatePageNum();
}



function scrollWin() {
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
    btn.innerHTML = (positionIndex+1)+" of "+elementArrayList.length;
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