const myData = await dioxus.recv();
const e = new CustomEvent("blochpointsupdate", { detail: myData });

document.dispatchEvent(e);
