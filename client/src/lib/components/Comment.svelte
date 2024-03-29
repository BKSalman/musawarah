<script lang="ts">
    import { goto } from "$app/navigation";
    import type { ComicCommentResponse } from "bindings/ComicCommentResponse";
    import { faAngleDown, faMessage } from "@fortawesome/free-solid-svg-icons";
    import Fa from "svelte-fa";
    import { currentUser } from "../../routes/stores";
    import { fly } from "svelte/transition";
    import type { ChapterCommentResponse } from "bindings/ChapterCommentResponse";

    export let comment: ComicCommentResponse | ChapterCommentResponse;
    export let indent = 0;
    let openChildren = comment.parent_comment == null;
    let openReplyBox = false;
    let inputComment = "";

    function isComicComment(object: any): object is ComicCommentResponse {
        return 'comic_id' in object;
    }

    async function sendComment(parent_comment_id: string) {
        if (inputComment.length < 1) {
            return;
        }

        const route = isComicComment(comment) ? `http://localhost:6060/api/v1/comics/${comment.comic_id}/comments` : `http://localhost:6060/api/v1/comics/chapters/${comment.chapter_id}/comments`;

        const res = await fetch(route, {
            credentials: "include",
            method: "POST",
            headers: {
              "Content-Type": "application/json",
            },
            body: JSON.stringify({
                content: inputComment,
                parent_comment_id: parent_comment_id
            }),
        });

        if (res.status >= 400) {
            // TODO: add message to the user that they need to log in
            await goto("/login");
        } else {
            if (isComicComment(comment)) {
                comment.child_comments.push(await res.json() as ComicCommentResponse);
            } else {
                comment.child_comments.push(await res.json() as ChapterCommentResponse);
            }
            // force it to refresh
            comment = comment;
            openChildren = true;
            openReplyBox = false;
            inputComment = "";
        }
    }

    function focus(el: HTMLElement) {
        el.focus()
    }

</script>

<div transition:fly|global={{ y: -10, duration: 100 }} class="comment" style="padding-left: {indent}rem">
    <div class="">{comment.user.username}</div>
    <div class="">{comment.content}</div>
    <button class="reply-box-collapse-button" on:click={() => openReplyBox = !openReplyBox}>
        <Fa size="1.5x" icon={faMessage} />
    </button>
    {#if openReplyBox}
        {#if $currentUser}
            <div transition:fly|global={{ y: -10, duration: 100 }} class="new-reply">
                <input type="text" name="comment" placeholder="Add a reply"
                    bind:value={inputComment}
                    use:focus
                    on:keypress={(e) => { if (e.key === "Enter") sendComment(comment.id) }}>

                <button on:click={() => sendComment(comment.id)}>send</button>
            </div>
        {:else}
            <h3 transition:fly|global={{ y: 10, duration: 200 }}><a href="/login">need to be logged in</a></h3>
        {/if}
    {/if}
    {#if comment.child_comments.length > 0}
        <div class="children">
            <button class="children-collapse-button" on:click={() => openChildren = !openChildren}>
                <Fa size="1.5x" icon={faAngleDown} />
            </button>
            {#if openChildren}
                {#each comment.child_comments || [] as child}
                    <svelte:self comment={child} indent={indent + 0.1}/>
                {/each}
            {/if}
        </div>
    {/if}
</div>

<style>
.children {
    margin-top: 10px;
    margin-left: 15px;
}
.comment {
    background: #AAAAAA;
    margin-bottom: 10px;
    border-radius: 5px;
    width: 90%;
}
.children-collapse-button {
    background: none;
    border: none;
    cursor: pointer;
}
.reply-box-collapse-button {
    background: none;
    border: none;
    cursor: pointer;
}
</style>
