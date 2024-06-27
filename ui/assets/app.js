const body = document.body;
const uploadBackdrop = document.getElementById("upload");
const modsContainer = document.getElementById("mods");
let mods = [];

__TAURI__.event.listen("file", event => {
    switch (event.payload) {
        case "hovered":
            uploadBackdrop.classList.add("active");
            break;
        case "drop":
        case "cancelled":
            uploadBackdrop.classList.remove("active");
            break;
    }
});

__TAURI__.event.listen("mods", event => {
    mods = event.payload.mods;

    load();
});

function load() {
    modsContainer.innerHTML = "";

    mods.forEach(mod => {
        const p = document.createElement("p");
        p.textContent = mod.name;

        const div = document.createElement("div");
        div.classList.add("mod");
        div.append(p);

        modsContainer.append(div);
    });
}

function get_mods() {
    __TAURI__.invoke("get_mods");
}

setTimeout(get_mods, 2000);