const events = ["abort", "afterprint", "animationend", "animationiteration", "animationstart", "beforeprint", "beforeunload", "blur", "canplay", "canplaythrough", "change", "click", "contextmenu", "copy", "cut", "dblclick", "drag", "dragend", "dragenter", "dragleave", "dragover", "dragstart", "drop", "durationchange", "ended", "error", "focus", "focusin", "focusout", "fullscreenchange", "fullscreenerror", "hashchange", "input", "invalid", "keydown", "keypress", "keyup", "load", "loadeddata", "loadedmetadata", "loadstart", "message", "mousedown", "mouseenter", "mouseleave", "mousemove", "mouseover", "mouseout", "mouseup", "mousewheel", "offline", "online", "open", "pagehide", "pageshow", "paste", "pause", "play", "playing", "popstate", "progress", "ratechange", "resize", "reset", "scroll", "search", "seeked", "seeking", "select", "show", "stalled", "storage", "submit", "suspend", "timeupdate", "toggle", "touchcancel", "touchend", "touchmove", "touchstart", "transitionend", "unload", "volumechange", "waiting", "wheel"];
let socket = new WebSocket(location.protocol === 'https:' ? "wss://" : "ws://" + window.location.hostname + ":2794");
for (var i = 0; i < events.length; i++) {
    window.addEventListener(events[i], function (event) {
        socket.send("{\"event_name\":\"" + event.type + "\",\"event\":" + stringify_object(event) + "}");
    })
}

function stringify_object(object, depth = 0, max_depth = 2) {
    // change max_depth to see more levels, for a touch event, 2 is good
    if (depth > max_depth)
        return 'Object';

    const obj = {};
    for (let key in object) {
        let value = object[key];
        if (value instanceof Node)
            // specify which properties you want to see from the node
            value = {id: value.id};
        else if (value instanceof Window)
            value = 'Window';
        else if (value instanceof Object)
            value = stringify_object(value, depth + 1, max_depth);

        obj[key] = value;
    }

    return depth ? obj : JSON.stringify(obj);
}

socket.onmessage = function (event) {
    let data = JSON.parse(event.data);
    if (data.name === "js") {
        eval(data.data);
    } else if (data.name === "html") {
        document.querySelector("body").innerHTML = data.data;
    }
};