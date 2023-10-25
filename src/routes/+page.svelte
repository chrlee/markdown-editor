<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';

  let markdown = "";
  let html = "";
  async function md_to_html(){
    html = await invoke("md_to_html", { markdown });
  }
</script>

<main class="container">
  <div class="editor">
    <textarea bind:value={markdown} on:input={md_to_html} />
  </div>
  <div class="preview">
    {@html html}
  </div>
</main>

<style>
  textarea {
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
  }
</style>
