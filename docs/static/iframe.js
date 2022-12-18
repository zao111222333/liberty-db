if (document.getElementById("pdf_iframe_module")==null){
    let iframeModule = document.createElement('script');
    iframeModule.src = "https://zao111222333.github.io/liberty-rs/switch.js";
    iframeModule.id = "pdf_iframe_module";
    document.body.insertAdjacentElement("afterend", iframeModule);
}