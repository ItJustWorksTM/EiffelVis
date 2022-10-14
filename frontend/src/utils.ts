export const deep_copy = (o: object) => {
    const out = Array.isArray(o) ? [] : {};
    for (const key in o) {
        const v = o[key];
        out[key] = (typeof v === "object" && v !== null) ? deep_copy(v) : v;
    }
    return out;
}
