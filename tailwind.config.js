/** @type {import('tailwindcss').Config} */
export default {
    content: ['./src/**/*.{html,js,svelte,ts}'],
    theme: {
        colors: {
            transparent: 'transparent',
            paper: {
                1: 'var(--paper1)',
                2: 'var(--paper2)',
                3: 'var(--paper3)',
                4: 'var(--paper4)',
                5: 'var(--paper5)',
                6: 'var(--paper6)',
                7: 'var(--paper7)',
                8: 'var(--paper8)',
                9: 'var(--paper9)',
                10: 'var(--paper10)',
            },
            ink: {
                1: 'var(--ink1)',
                2: 'var(--ink2)',
                3: 'var(--ink3)',
                4: 'var(--ink4)',
            },
        },
        extend: {
            fontSize: {
                inherit: 'inherit',
            },
        },
    },
    plugins: [require('@tailwindcss/forms')],
};
