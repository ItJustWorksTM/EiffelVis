export const deep_copy = (o: object) => {
    let out, v, key;
    out = Array.isArray(o) ? [] : {};
    for (key in o) {
        v = o[key];
        out[key] = (typeof v === "object" && v !== null) ? deep_copy(v) : v;
    }
    return out;
}
