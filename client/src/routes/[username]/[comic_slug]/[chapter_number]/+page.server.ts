import { error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import type { ChapterResponse } from 'bindings/ChapterResponse';

export const load = (async ({ fetch, params }) => {
    const { comic_slug, username, chapter_number } = params;

    const res = await fetch(`http://localhost:6060/api/v1/comics/chapters/by_slug/${username}/${comic_slug}/${chapter_number}/`);

    if (res.status != 200) {
        const res_error = await res.json().catch(() => ({ error: res.statusText }));
        throw error(res.status, res_error);
    }

    const data: ChapterResponse = await res.json();

    return {
        chapter: data
    };

}) satisfies PageServerLoad;
