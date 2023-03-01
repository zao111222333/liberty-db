try{
pdf2htmlEX.defaultViewer = new pdf2htmlEX.Viewer({});
}catch(e){}

const color = 'rgba(227,238,0,0.2)';
const selectColor = 'rgba(180,0,170,0.15)';
let pagePosition=0;
let isScrolling;
let elementArrayList = [];
let pagePositionList = [];
let pageNum = 1;
let positionIndex = 0;
let page;
let container;
let srollBtn;
let pageInput;
var queryCtx = {
    field: "",
    bgnList: [],
    endList: [],
};
function updateQueryCtx(queryString){
    let params = new URLSearchParams(queryString);
    queryCtx.field = params.get('field');
    let bgnParams = params.get('bgn');
    let endParams = params.get('end');
    if (bgnParams != null && endParams != null){
        queryCtx.bgnList = bgnParams.split(' ');
        queryCtx.endList = endParams.split(' ');
    }
}
function queryString(){
    let s = '?';
    if (queryCtx.field!=''){
        s+='field='+queryCtx.field+'&';
    }
    let bgnParams='';
    if (queryCtx.bgnList.length!=0){
        bgnParams = "bgn="
        for (let index = 0; index < queryCtx.bgnList.length; index++) {
            bgnParams += queryCtx.bgnList[index]+"+"
        }
        s+=bgnParams.slice(0, -1)+'&'; 
    }
    let endParams='';
    if (queryCtx.endList.length!=0){
        endParams = "end="
        for (let index = 0; index < queryCtx.endList.length; index++) {
            endParams += queryCtx.endList[index]+"+"
        }
        s+=endParams.slice(0, -1)+'&';
    }
    if (s=='?'){
        return ''
    }else{
        return s.slice(0, -1)
    }
}

container = document.getElementById('page-container');
container.onscroll = (_) => {
    window.clearTimeout( isScrolling );
    isScrolling = setTimeout(function() {
        updatePageNum();
    }, 50);
};
updatePagePosition();
let bgDiv = document.createElement("div");
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
let Div = document.createElement("div");
Div.style.display = "flex";
Div.style.justifyContent = "space-between";
Div.style.flexDirection = "row-reverse";

let observer = new MutationObserver(function(mutations) {
    updatePagePosition();
    updateDivWidth();
});
observer.observe(container.firstElementChild, { attributes : true, attributeFilter : ['style'] });
function updateDivWidth(){
    let style = getComputedStyle(container.firstElementChild);
    if (style!=null){
        Div.style.width=style.width;
    }
}

container.firstElementChild.appendChild(bgDiv);
bgDiv.appendChild(Div);
container.firstElementChild.style.marginTop = '28px';
let pageDiv = document.createElement("div");
pageDiv.style.flexDirection = "row";
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
pageInput.style.marginTop = "1px";
pageInput.style.marginBottom = "2px";
pageInput.style.marginRight = "-5px";
pageInput.style.textAlign = "right";
pageInput.addEventListener("keydown", (event) => {
    if (event.isComposing || event.keyCode === 229) {
      return;
    }
    if(event.key === 'Enter') {
        let num = parseInt(pageInput.value);
        if (num<=0){
            num = 1;
        }
        if (num>pagePositionList.length){
            num=pagePositionList.length
        }
        pageInput.value=num;
        container.scrollTop = pagePositionList[num-1];
    }
  })
pageDiv.appendChild(pageInput);
let pageText = document.createElement("input");
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
let btnDiv = document.createElement("div");
addSidebarBtn();
btnDiv.appendChild(srollBtn);
Div.appendChild(btnDiv);

let removeBtn = document.getElementById('remove_btn');
if (removeBtn==null){
    removeBtn = document.createElement("button");
    removeBtn.id = 'remove_btn';
    let innerText = document.createElement('a');
    innerText.innerHTML+='Remove';
    removeBtn.appendChild(innerText);
    removeBtn.style.cursor = 'pointer';
    removeBtn.style.color = '#ffffff';
    removeBtn.style.backgroundColor = 'rgb(63,63,63)';
    btnDiv.appendChild(removeBtn);
    removeBtn.addEventListener("click",removeOne)
}

function removeOne(){
    if (elementArrayList.length>0){
        disHighlight([elementArrayList[positionIndex]])
        elementArrayList.splice(positionIndex, 1);
        queryCtx.bgnList.splice(positionIndex, 1);
        queryCtx.endList.splice(positionIndex, 1);
        updateLinkBtn();
        if (positionIndex !=0){
            positionIndex --
        }
        if (elementArrayList.length==0){
            srollBtn.innerHTML = "None";
        }else{
            srollBtn.innerHTML = (positionIndex+1)+" of "+elementArrayList.length;
        }
    }
}

let linkBtn = document.getElementById('link_btn');
if (linkBtn==null){
    linkBtn = document.createElement("button");
    linkBtn.id = 'link_btn';
    let innerText = document.createElement('a');
    innerText.innerHTML+='Copy';
    let linkSymbol = `<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-link-45deg" viewBox="0 0 16 16" data-darkreader-inline-fill="" style="--darkreader-inline-fill:currentColor;">
    <path d="M4.715 6.542 3.343 7.914a3 3 0 1 0 4.243 4.243l1.828-1.829A3 3 0 0 0 8.586 5.5L8 6.086a1.002 1.002 0 0 0-.154.199 2 2 0 0 1 .861 3.337L6.88 11.45a2 2 0 1 1-2.83-2.83l.793-.792a4.018 4.018 0 0 1-.128-1.287z"></path>
    <path d="M6.586 4.672A3 3 0 0 0 7.414 9.5l.775-.776a2 2 0 0 1-.896-3.346L9.12 3.55a2 2 0 1 1 2.83 2.83l-.793.792c.112.42.155.855.128 1.287l1.372-1.372a3 3 0 1 0-4.243-4.243L6.586 4.672z"></path>
    </svg>`;
    innerText.innerHTML+=linkSymbol;
    innerText.innerHTML+='&nbsp;&nbsp;&nbsp;';
    linkBtn.appendChild(innerText);
    linkBtn.style.cursor = 'pointer';
    linkBtn.style.color = '#ffffff';
    linkBtn.style.backgroundColor = 'rgb(63,63,63)';
    btnDiv.appendChild(linkBtn);
}

let appendBtn = document.getElementById('append_btn');
if (appendBtn==null){
    appendBtn = document.createElement("button");
    appendBtn.id = 'append_btn';
    let innerText = document.createElement('a');
    innerText.innerHTML+='Append';
    appendBtn.appendChild(innerText);
    appendBtn.style.cursor = 'pointer';
    appendBtn.style.color = '#ffffff';
    appendBtn.style.backgroundColor = 'rgb(63,63,63)';
    btnDiv.appendChild(appendBtn);
    appendBtn.style.display = 'none';
}
// var _baseNode;
// var _extentNode;
document.onselectionchange = function(){
    if (window.getSelection().type == "Range"){
        appendBtn.style.display = '';
        appendBtn.removeEventListener("click", ()=>{})
        appendBtn.addEventListener("click",function listener(){
            function getIdOffsite(node){
                if (node.id == undefined | node.id ==''){
                    let parent = node.parentElement
                    if (document.body === parent){
                        return ["worn_id",0];
                    }else{
                        return getIdOffsite(parent);
                    }
                }
                return [node.id,node.offsetTop+node.parentElement.parentElement.offsetTop];
            }
            if (window.getSelection().type == "Range"){
                let bgnId;
                let endId;
                let [baseId,baseOffset] = getIdOffsite(window.getSelection().baseNode);
                let [extentId,extentOffset] = getIdOffsite(window.getSelection().extentNode);
                if (baseOffset<extentOffset){
                    bgnId = baseId
                    endId = extentId
                }else{
                    bgnId = extentId
                    endId = baseId
                }
                queryCtx.bgnList.push(bgnId);
                queryCtx.endList.push(endId);
                updateLinkBtn();
                let elementArray = [];
                let hasErr = highlight(document.getElementById(bgnId),endId,elementArray);
                if (!hasErr){
                    elementArrayList.push(elementArray);
                    positionIndex=0;
                    srollBtn.innerHTML = (positionIndex+1)+" of "+elementArrayList.length;
                    srollBtn.addEventListener("click", scrollWin);
                    removeBtn.addEventListener("click", removeOne);
                    appendBtn.style.display = 'none';
                }
                window.getSelection().removeAllRanges();
            }
        })
    }else{
        appendBtn.style.display = 'none';
    }
};


function updateLinkBtn() {
    linkBtn.addEventListener("click",()=>{
        let linkUrl = new URL(window.location.href);
        linkUrl.search = queryString();
        navigator.clipboard.writeText(linkUrl.toString());
    })   
}
window.onmessage = function(e) {
    updateQueryCtx(e.data);
    updateQuery();
    updateLinkBtn();
};
window.onload = function(){
    updateQueryCtx(window.location.search);
    updateDivWidth();
    updateQuery();
    updateLinkBtn();
} 
function updateQuery() {
    disHighlight(elementArrayList);
    elementArrayList = [];
    if (queryCtx.bgnList.length==queryCtx.endList.length){
        for (let index = 0; index < queryCtx.bgnList.length; index++) {
            const bgn = queryCtx.bgnList[index];
            const end = queryCtx.endList[index];
            let element = document.getElementById(bgn);
            let elementArray = [];
            highlight(element,end,elementArray);
            elementArrayList.push(elementArray);
        }
        positionIndex = elementArrayList.length-1;
    }else{
        console.error("length of Begin and Ended are NOT equal")
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
    let newPagePosition = element.parentElement.parentElement.offsetTop;
    let elementPosition = element.offsetTop;
    if (Math.abs(newPagePosition-pagePosition)<1000){
        container.scrollTo({top: elementPosition+newPagePosition-10, behavior: 'smooth'});
    }else{
        let newPagePositionTo;
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
                    container.scrollTo({top: elementPosition+newPagePosition-20, behavior: 'smooth'});
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
    try {
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
    } catch (e) {
        return true
    }
    return false
}
function updatePageNum(){
    const offset = container.scrollTop;
    pagePosition = offset;
    let num = pageNum;
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
function updatePagePosition(){
    pagePositionList=[];
    page = container.firstElementChild;
    pagePositionList.push(page.offsetTop);
    while (page.nextElementSibling!=null) {
        page = page.nextElementSibling;
        pagePositionList.push(page.offsetTop);
    }
}
function addSidebarBtn(){
    if (document.getElementById("outline").getElementsByTagName('ul').length!=0){
        let toggleSidebarBtn = document.createElement("button");
        let innerText = document.createElement('a');
        let linkSymbol = `<svg width="16" height="16" style="margin-bottom:-3px;margin-top:-1px;" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
        <path d="M7 3H2v14h5V3zm2 0v14h9V3H9zM0 3c0-1.1.9-2 2-2h16a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H2a2 2 0 0 1-2-2V3zm3 1h3v2H3V4zm0 3h3v2H3V7zm0 3h3v2H3v-2z"/>
        </svg>`;
        innerText.innerHTML+=linkSymbol;
        toggleSidebarBtn.appendChild(innerText);
        toggleSidebarBtn.style.cursor = 'pointer';
        toggleSidebarBtn.style.color = '#ffffff';
        toggleSidebarBtn.style.backgroundColor = 'rgb(63,63,63)';
        toggleSidebarBtn.addEventListener("click", function(){
            if (document.getElementById("sidebar").classList.toggle("opened")){
                toggleSidebarBtn.style.backgroundColor = 'rgb(53,53,53)';
            }else{
                toggleSidebarBtn.style.backgroundColor = 'rgb(63,63,63)';
            }
        });
        btnDiv.appendChild(toggleSidebarBtn);
    }
}