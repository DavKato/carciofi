<script>
    import { version } from '$app/environment';
</script>

<footer class="py-2 px-3 flex items-center justify-between">
    <div class="connection flex items-center gap-2">
        <div class="ollama-logo">(o)</div>
        <select name="list" id="list" class="bg-stone-700 border-none py-0 rounded-sm">
            <option value="temp">temp</option>
        </select>
        <div class="status">✅</div>
    </div>

    <p class="text-right italic text-xs">v{version}</p>
</footer>
