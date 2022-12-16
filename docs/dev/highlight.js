/*
Demo:
https://zao111222333.github.io/liberty-db/liberty07_03.html
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
const queryString = window.location.search;
const params = new URLSearchParams(queryString);
const field = params.get('field');
const bgnParams = params.get('bgn');
const endParams = params.get('end');
const color = 'rgba(227,238,0,0.2)';
const selectColor = 'rgba(180,0,170,0.15)';
const container = document.getElementById('page-container');
let elementArrayList = [];
let pagePositionList = [];
var pageNum = 1;
let page = container.firstElementChild;
pagePositionList.push(page.offsetTop);
while (page.nextElementSibling!=null) {
    page = page.nextElementSibling;
    pagePositionList.push(page.offsetTop);
}

var Div = document.createElement("div");
Div.style.display = "flex";
Div.style.justifyContent = "center";
document.body.appendChild(Div);
var DivDiv = document.createElement("div");
DivDiv.style.position = "fixed";
DivDiv.className = "w0";
DivDiv.style.marginTop = "-8px";
Div.appendChild(DivDiv);

var pageDiv = document.createElement("div");
pageDiv.style.position = "absolute";
pageDiv.style.right = "0px";
pageDiv.style.display = "flex";
pageDiv.style.flexDirection = "row";
pageDiv.style.backgroundColor = "white";
DivDiv.appendChild(pageDiv);
var pageInput = document.createElement("input");
pageInput.type = "text";
pageInput.style.width = "30px";
pageInput.style.zIndex = "2";
pageInput.style.marginRight = "-5px";
pageInput.style.textAlign = "right";
pageInput.setAttribute("onkeydown", "toPage(this)");
pageDiv.appendChild(pageInput);
var pageText = document.createElement("input");
pageText.style.width = "34px";
pageText.type = "text";
pageText.disabled = true;
pageText.value = " / "+pagePositionList.length;
pageDiv.appendChild(pageText);
var pagePosition=0;
var isScrolling;
container.onscroll = (_) => {
    window.clearTimeout( isScrolling );
    isScrolling = setTimeout(function() {
        updatePageNum();
    }, 50);
};

if (bgnParams != null && endParams != null){
    let bgnList = bgnParams.split(' ');
    let endList = endParams.split(' ');
    if (bgnList.length==endList.length){
        for (let index = 0; index < bgnList.length; index++) {
            const bgn = bgnList[index].replaceAll('.', ' ');
            const end = endList[index].replaceAll('.', ' ');
            let element = document.getElementsByClassName(bgn)[0];
            var elementArray = [];
            highlight(element,end,elementArray);
            elementArrayList.push(elementArray);
        }
        var btn = document.createElement("button");
        btn.setAttribute("onclick", "scrollWin()");
        DivDiv.appendChild(btn);
        var positionIndex = elementArrayList.length-1;
        scrollWin();
    }else{
        console.error("length of Begin and Ended are NOT equal")
    }
}
updatePageNum();

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
        delay(200).then(() => {
        elementPosition = element.offsetTop;
        container.scrollTo({top: elementPosition+newPagePosition-10, behavior: 'smooth'});
    });
    }
    pagePosition = newPagePosition;
}
function delay(time) {
  return new Promise(resolve => setTimeout(resolve, time));
}

function highlight(element, end, elementArray) {
    if (element!=null){
        if (element.className.substring(0,2)=='t '){
            element.style.backgroundColor = color;
            elementArray.push(element);
        }
        if (element.className!=end){
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
        container.scrollTop = pagePositionList[+(element.value)-1];
    }
}