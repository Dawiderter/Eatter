import { sveltekit } from '@sveltejs/kit/vite';
import type { UserConfig } from 'vite';

const config: UserConfig = {
	plugins: [sveltekit()],
	server: {
		host: "0.0.0.0",
		proxy: {
		  // string shorthand: http://localhost:5173/foo -> http://localhost:4567/foo
		  '/api': {
			target: "http://0.0.0.0:3000",
			rewrite: path => path.replace(/^\/api/, ''),
		  },
		}
	}
};

export default config;
