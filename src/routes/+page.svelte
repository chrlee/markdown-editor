<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';

  let markdown = "";
  let html = "";
  async function md_to_html(){
    html = await invoke("md_to_html", { markdown });
  }
  function syncScroll(e: Event){
    const preview = document.querySelector(".preview") as HTMLElement;
    const editor = document.querySelector("textarea") as HTMLElement;
    const offsetTop = editor.scrollTop / (editor.scrollHeight - editor.offsetHeight);
    const offsetLeft = editor.scrollLeft / (editor.scrollWidth - editor.offsetWidth);

    preview.scrollTop = offsetTop * (preview.scrollHeight - preview.offsetHeight);
    preview.scrollLeft = offsetLeft * (preview.scrollWidth - preview.offsetWidth);
  }
</script>

<main class="container">
  <div class="editor">
    <textarea bind:value={markdown} on:input={md_to_html} on:scroll={syncScroll}/>
  </div>
  <div class="preview">
    {@html html}
  </div>
</main>

<style>
  :global(textarea) {
    border: none;
    overflow: auto;
    outline: none;

    -webkit-box-shadow: none;
    -moz-box-shadow: none;
    box-shadow: none;

    resize: none;

    padding: 0;

    width: 100%;
    height: 100%;
  }

  .editor {
    flex-grow: 1;
    flex-basis: 0;
  }
  .preview {
    flex-grow: 1;
    flex-basis: 0;
    overflow: scroll;
  }
</style>
