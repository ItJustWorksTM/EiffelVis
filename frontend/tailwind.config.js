module.exports = {
    future: {},
    content: ['./public/index.html', './src/**/*.svelte'],
    theme: {
        extend: {},
    },
    variants: {},
    plugins: [require('daisyui')],

    daisyui: {
        styled: true,
        themes: ['halloween'],
        base: true,
        utils: true,
        logs: true,
    },
};
