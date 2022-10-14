module.exports = {
    plugins: [
        '@typescript-eslint', 'svelte'
    ],

    root: true,
    parser: "@typescript-eslint/parser",

    parserOptions: {
        ecmaVersion: 'latest',
        sourceType: 'module',
    },

    env: {
        browser: true,
        commonjs: true,
        es6: true,
        node: true
    },

    extends: [
        'eslint:recommended',
        'plugin:@typescript-eslint/eslint-recommended',
        'plugin:@typescript-eslint/recommended'
    ],

    overrides: [
        {
            files: ['*.svelte'],
            parser: 'svelte-eslint-parser',
            parserOptions: {
                parser: "@typescript-eslint/parser",
            },
        }
    ],

    rules: {
        '@typescript-eslint/ban-types': 'warn',
        '@typescript-eslint/no-empty-function': 'warn',
        '@typescript-eslint/no-empty-interface': 'off',
        '@typescript-eslint/no-inferrable-types': 'off',
        "@typescript-eslint/no-unused-vars": [
            'warn',
            {
                argsIgnorePattern: '^_$',
                varsIgnorePattern: '^_$',
                caughtErrorsIgnorePattern: '^_$'
            }
        ]
    },
}
