<script lang="ts">
	import UrlInput from '../lib/components/Urlnput.svelte'
  </script>
  
  <main class="container">
	<h1>Interclip</h1>
	<p>
	  Input a URL <s>or drag n' drop an image</s>.
	</p>
  
	<div class="row">
	  <UrlInput />
	</div>
  </main>


<style>
	section {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		flex: 0.6;
	}

	h1 {
		width: 100%;
	}

	.welcome {
		display: block;
		position: relative;
		width: 100%;
		height: 0;
		padding: 0 0 calc(100% * 495 / 2048) 0;
	}

	.welcome img {
		position: absolute;
		width: 100%;
		height: 100%;
		top: 0;
		display: block;
	}
</style>
