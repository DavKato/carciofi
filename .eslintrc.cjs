/** @type { import("eslint").Linter.Config } */
module.exports = {
    root: true,
    extends: [
        'eslint:recommended',
        'plugin:@typescript-eslint/recommended',
        'plugin:svelte/recommended',
        'prettier',
    ],
    parser: '@typescript-eslint/parser',
    plugins: ['@typescript-eslint', 'prettier'],
    parserOptions: {
        sourceType: 'module',
        ecmaVersion: 2020,
        extraFileExtensions: ['.svelte'],
    },
    env: {
        browser: true,
        es2017: true,
        node: true,
    },
    ignorePatterns: [
        '.DS_Store',
        'node_modules',
        'build',
        '.svelte-kit',
        'package',
        '.env',
        '.env.*',
        '!.env.example',
        'pnpm-lock.yaml',
        'package-lock.json',
        'yarn.lock',
        'src-tauri',
    ],
    overrides: [
        {
            files: ['*.svelte'],
            parser: 'svelte-eslint-parser',
            parserOptions: {
                parser: '@typescript-eslint/parser',
            },
        },
    ],
    rules: {
        'prettier/prettier': 'error',
    },
};
