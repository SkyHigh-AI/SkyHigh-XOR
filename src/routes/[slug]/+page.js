/** @type {import('./$types').PageLoad} */
export function load({ params }) {
	return {
		ai: {
            path: params.slug
		}
	};
}

export const prerender = false;
