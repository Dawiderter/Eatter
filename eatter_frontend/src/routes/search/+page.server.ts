import { get } from 'svelte/store'
import { tags } from '$lib/stores.js'

import type { PageServerLoad } from "./$types";
import { fetch_meals } from '$lib/search';

export const load = (async (event) => {
    console.log("fetching");

    const meals = await fetch_meals(event, ['burger']);

    return {
        meals: meals
    }

}) satisfies PageServerLoad;