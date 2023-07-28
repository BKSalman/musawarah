import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import type { ComicGenre } from '../../../../bindings/ComicGenre';

export const load = (async ({ fetch }) => {
    const res = await fetch("http://localhost:6060/api/v1/comics/genres");

    if (res.status !== 200) {
        const res_error = await res.json().catch(() => ({ error: res.statusText }));
        throw error(res.status, res_error);
    }

    const genres: ComicGenre[] = await res.json();

    return {
        genres: genres,
    }
}) satisfies PageLoad;

