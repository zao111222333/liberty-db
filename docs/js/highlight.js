// https://convertio.co/pdf-html/
const queryString = window.location.search;
const params = new URLSearchParams(queryString);
const field = params.get('field');
const beginParams = params.get('begin');
const endedParams = params.get('ended');
const color = 'rgba(255,0,0,0.1)'
const selectColor = 'rgba(255,0,0,0.3)'

let elementArrayList = [];
// let positionList = [];
if (beginParams != null && endedParams != null){
    let beginList = beginParams.split(' ');
    let endedList = endedParams.split(' ');
    if (beginList.length==endedList.length){
        for (let index = 0; index < beginList.length; index++) {
            const begin = beginList[index].replaceAll('.', ' ');
            const ended = endedList[index].replaceAll('.', ' ');
            let element = document.getElementsByClassName(begin)[0];
            var elementArray = [];
            highlight(element,ended,elementArray);
            elementArrayList.push(elementArray);
            // element.style.backgroundColor = firstColor;
        }
    }else{
        console.error("length of Begin and Ended are NOT equal")
    }
}
// .w0{width:612.000000px;}
const container = document.getElementById('page-container');
var btnDiv = document.createElement("div");
var btnDivDiv = document.createElement("div");
btnDiv.style.display = "flex";
btnDiv.style.justifyContent = "center";
btnDivDiv.className = "w0";
document.body.appendChild(btnDiv);
btnDiv.appendChild(btnDivDiv);
var btn = document.createElement("button");
btn.setAttribute("onclick", "scrollWin()");
btn.style.position = "fixed";
btn.style.marginLeft = "-25px";
btnDivDiv.appendChild(btn);
var positionIndex = elementArrayList.length-1;
scrollWin();

function scrollWin() {
    for (let index = 0; index < elementArrayList[positionIndex].length; index++) {
        const element = elementArrayList[positionIndex][index];
        element.style.backgroundColor = color;
    }
    // elementArrayList[positionIndex].style.backgroundColor = color;
    if (positionIndex == elementArrayList.length-1){
        positionIndex = 0;
    }else{
        positionIndex = positionIndex + 1;
    }
    const firstElement = elementArrayList[positionIndex][0];
    for (let index = 0; index < elementArrayList[positionIndex].length; index++) {
        const element = elementArrayList[positionIndex][index];
        element.style.backgroundColor = selectColor;
    }
    btn.innerHTML = (positionIndex+1)+" of "+elementArrayList.length;
    // console.log(element);
    // console.log(element.offsetParent);
    var elementPosition = firstElement.offsetTop;
    // console.log(elementPosition);
    var pagePosition = firstElement.parentElement.parentElement.offsetTop;
    // console.log(pagePosition);
    container.scrollTop = elementPosition+pagePosition-10;
}

function highlight(element, ended, elementArray) {
    if (element!=null){
        if (element.className.substring(0,2)=='t '){
            element.style.backgroundColor = color;
            elementArray.push(element);
        }
        if (element.className!=ended){
            let next = element.nextElementSibling;
            if (next==null){
                next = element.parentElement.parentElement.nextElementSibling.firstChild.firstChild;
            }
            highlight(next,ended,elementArray);
        }
    }
}
/*
file:///Users/junzhuo/Developer/EDA/liberty-db/docs/html/liberty07_03.html
?field=test
&begin
=t.m0.x9.hc.ya0.ff7.fs2.fc2.sc0.ls0
+t.m0.x2.h5.y28.ff1.fs2.fc2.sc0.ls0.ws0
+t.m0.x2.h5.y38.ff1.fs2.fc2.sc0.ls0.ws0
+t.m0.x2.h5.y55.ff1.fs2.fc2.sc0.ls0.ws0
+t.m0.x2.h6.y13.ff1.fs2.fc2.sc0.ls0.ws0
&ended
=t.m0.x9.hc.ya0.ff7.fs2.fc2.sc0.ls0
+t.m0.x2.h5.y2d.ff1.fs2.fc2.sc0.ls0.ws0
+t.m0.x2.h5.y4a.ff1.fs2.fc2.sc0.ls0.ws0
+t.m0.x2.h5.y55.ff1.fs2.fc2.sc0.ls0.ws0
+t.m0.x2.h6.y13.ff1.fs2.fc2.sc0.ls0.ws0
*/