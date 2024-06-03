import { fontFamily } from "tailwindcss/defaultTheme";
import { addDynamicIconSelectors } from "@iconify/tailwind";

/** @type {import('tailwindcss').Config} */
export const darkMode = "class";
export const content = ["./templates/*.html"];
export const theme = {
    container: {
        center: true,
    },
    extend: {
        fontFamily: {
            serif: ["Merriweather", ...fontFamily.serif],
            sans: ["Inter var", ...fontFamily.sans],
        },
    },
};
export const plugins = [addDynamicIconSelectors()];
