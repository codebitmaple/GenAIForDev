/** @type {import('tailwindcss').Config} */
module.exports = {
	content: ["./assets/**/*.{hbs,css}", "./src/**/*.{hbs,rs}"],
	theme: {
		extend: {
			fontFamily: {
				body: ["Noto Sans", "sans-serif"], // Add your custom font here
			},
			colors: {
				// Rich Royal Purple Shades
				"brand-deep-purple": "#5D3FD3",
				"brand-imperial-violet": "#6000A0",
				"brand-electric-purple": "#7D00FF",
				"brand-dark-amethyst": "#663399",
				"brand-midnight-purple": "#4B0082",

				// Pastel Colors Based on Royal Purple
				"brand-pastel-purple": "#B39CD0",
				"brand-lavender-mist": "#D6C3E0",
				"brand-dusty-lilac": "#C8A2C8",
				"brand-blush-pink": "#F4C2C2",
				"brand-periwinkle-whisper": "#C3CDE6",
				"brand-muted-mauve": "#D8A7B1",
				"brand-pale-gold": "#E8D8B3",

				// Additional Vibrant Color
				"brand-vibrant-gold": "#FFD400",
			},
		},
	},
	plugins: [],
};
