<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { toast } from "vue-sonner";
import { FolderSearch } from "@lucide/vue";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import Hint from "@/components/Hint.vue";
import TakeoverStep from "@/components/add-project-dialog/TakeoverStep.vue";
import AddManualForm from "@/components/add-project-dialog/AddManualForm.vue";
import ScanResultsPanel from "@/components/add-project-dialog/ScanResultsPanel.vue";
import {
  browseForFolder,
  scanForDevWebUI,
  suggestDest,
  type AddResult,
  type AutostartTrigger,
  type ProjectProposal,
  type ScanResult,
  type TakeOverResult,
} from "@/api";
import { useAppStore } from "@/store";
import { findProjectNameInDir, pathFromDrop, projectNameFromFile } from "@/lib/drop";

const open = defineModel<boolean>("open", { required: true });

const store = useAppStore();
const { t } = useI18n({ useScope: "global" });

const input = ref(""); // a local path OR a git URL
const dest = ref(""); // git clone destination
const error = ref("");
const note = ref(""); // friendly prompt when a drop can't reveal its path
const busy = ref(false);
// Set when a folder has no .devwebui but the daemon detected dev scripts to build one from.
const scaffold = ref<{ dir: string; fileName: string; proposal: ProjectProposal } | null>(null);
// Set after a load when the repo ALSO auto-starts its dev server outside DevWebUI
// (VS Code tasks.json folderOpen / the "Vite" extension) — offer to retire those.
const takeover = ref<{ dir: string; triggers: AutostartTrigger[]; result?: TakeOverResult } | null>(
  null,
);
// Folder-scan results (existing .devwebui files + detectable package-script projects),
// for ONE folder the user explicitly named — there is no machine-wide scan in this build.
const scanning = ref(false);
const scanResult = ref<ScanResult | null>(null);
let scanGen = 0; // bumped on reset/new-scan so a stale in-flight scan can't write back
// The folder the most recent scoped scan actually ran against, shown as a caption
// over the results so it's obvious what was searched.
const scannedFolder = ref("");
// The "scan a specific folder" card's input (a typed/pasted path).
const scanFolder = ref("");
async function ignoreDetected(dir: string) {
  try {
    await store.ignoreProject(dir);
  } catch {
    /* best-effort */
  }
}
async function unignoreDetected(dir: string) {
  try {
    await store.unignoreProject(dir);
  } catch {
    /* best-effort */
  }
}

// A real git remote has a scheme (https/ssh/git) or scp-style host. A bare
// ".git" suffix is deliberately NOT enough — local paths like C:\repos\x.git
// must still load as folders, not clone targets.
const GIT_RE = /^(https?:\/\/|git@[^:\s]+:|ssh:\/\/|git:\/\/)/i;
const isGitUrl = computed(() => GIT_RE.test(input.value.trim()));

// Drag highlight via a depth counter so moving over child elements doesn't flicker.
const dragDepth = ref(0);
const dragging = computed(() => dragDepth.value > 0);
let dropGen = 0; // ignore a slow drop-read once a newer drop has started

// Reset every time the dialog opens, and fetch a default clone destination.
watch(open, async (v) => {
  if (!v) return;
  input.value = "";
  dest.value = "";
  error.value = "";
  note.value = "";
  scaffold.value = null;
  takeover.value = null;
  scanResult.value = null;
  scanning.value = false;
  scannedFolder.value = "";
  scanFolder.value = "";
  scanGen++; // invalidate any scan still running from a previous open
  dragDepth.value = 0;
  busy.value = false;
  const suggested = await suggestDest();
  if (open.value && !dest.value) dest.value = suggested; // don't clobber a closed dialog or typed value
});

function clearMessages() {
  error.value = "";
  note.value = "";
  scaffold.value = null;
  takeover.value = null;
  scanResult.value = null;
  scannedFolder.value = "";
}

function finish(res: AddResult) {
  if (res.cancelled) return;
  if (res.needsScaffold && res.proposal && res.dir && res.fileName) {
    // No .devwebui here, but we detected dev scripts — offer to build one.
    scaffold.value = { dir: res.dir, fileName: res.fileName, proposal: res.proposal };
    return;
  }
  if (res.cloned && res.error) {
    // Repo cloned fine but has no .devwebui — recoverable. Point the user at it.
    note.value = res.error;
    input.value = res.cloned;
    return;
  }
  if (res.error) {
    error.value = res.error;
    scaffold.value = null; // a failed scaffold attempt is no longer actionable — drop the preview
    return;
  }
  if (res.ok && res.autostartTriggers?.length && res.dir) {
    // Project added — but it also auto-starts its dev server outside DevWebUI.
    // Keep the dialog open to offer retiring those triggers (it's already added).
    takeover.value = { dir: res.dir, triggers: res.autostartTriggers };
    if (res.firstLoad) toast.success(t("addProject.firstLoadHint"));
    return;
  }
  if (res.ok) {
    // A brand-new path never auto-starts anything on its first load — the dialog
    // closes right below (no blocking step), so this is a toast, not inline copy.
    if (res.firstLoad) toast.success(t("addProject.firstLoadHint"));
    open.value = false;
  }
}

async function createScaffold() {
  const s = scaffold.value;
  if (!s || busy.value) return;
  error.value = "";
  note.value = "";
  busy.value = true;
  let res: AddResult | undefined;
  try {
    res = await store.scaffoldProject(s.dir, s.fileName, {
      name: s.proposal.name,
      processes: s.proposal.processes,
    });
  } catch (e) {
    error.value = e instanceof Error ? e.message : t("addProject.requestFailed");
  } finally {
    busy.value = false;
  }
  if (res) finish(res);
}

// ---- scan ONE explicitly named folder ------------------------------------
// The ONLY scan this build offers: the folder from the dedicated card (typed or
// picked). One thorough, bounded pass of that folder — never the whole machine.
async function runScan(explicitRoot: string) {
  if (scanning.value || !explicitRoot) return;
  clearMessages();
  scanning.value = true;
  const gen = ++scanGen; // invalidate this run if the dialog is reset/reopened meanwhile
  try {
    scannedFolder.value = explicitRoot;
    const r = await scanForDevWebUI({
      roots: [explicitRoot],
      preset: "scoped",
      detectPackages: true,
    });
    if (gen === scanGen) scanResult.value = r;
  } catch {
    if (gen === scanGen) error.value = t("addProject.scanFailed");
  } finally {
    if (gen === scanGen) scanning.value = false;
  }
}

// Native folder picker, scoped straight into a scan.
async function browseScanFolder() {
  if (busy.value || scanning.value) return;
  try {
    const r = await browseForFolder();
    if (r.ok && r.path) void runScan(r.path);
  } catch {
    error.value = t("addProject.errPicker");
  }
}

// Enter in the "scan a specific folder" field — scan whatever was typed/pasted there.
function scanTypedFolder() {
  const f = scanFolder.value.trim();
  if (f) void runScan(f);
}

async function addFound(file: string) {
  if (busy.value) return;
  error.value = "";
  note.value = "";
  busy.value = true;
  let res: AddResult | undefined;
  try {
    res = await store.loadProjectByPath(file);
  } catch (e) {
    error.value = e instanceof Error ? e.message : t("addProject.requestFailed");
  } finally {
    busy.value = false;
  }
  if (res) finish(res);
}

async function submit() {
  if (busy.value) return;
  const val = input.value.trim();
  if (!val) {
    error.value = t("addProject.errPasteOrDrag");
    return;
  }
  if (isGitUrl.value && !dest.value.trim()) {
    error.value = t("addProject.errChooseDest");
    return;
  }
  clearMessages();
  busy.value = true;
  let res: AddResult | undefined;
  try {
    res = isGitUrl.value
      ? await store.cloneProject(val, dest.value.trim())
      : await store.loadProjectByPath(val);
  } catch (e) {
    error.value = e instanceof Error ? e.message : t("addProject.requestFailed");
  } finally {
    busy.value = false;
  }
  if (res) finish(res); // after busy clears — a programmatic close is blocked while busy
}

async function browseFile() {
  if (busy.value) return;
  clearMessages();
  busy.value = true;
  let res: AddResult | undefined;
  try {
    res = await store.browseForProject();
  } catch (e) {
    error.value = e instanceof Error ? e.message : t("addProject.requestFailed");
  } finally {
    busy.value = false;
  }
  if (res) finish(res);
}

async function pickDest() {
  if (busy.value) return;
  try {
    const r = await browseForFolder();
    if (r.ok && r.path) dest.value = r.path;
  } catch {
    error.value = t("addProject.errPicker");
  }
}

// ---- drag & drop: best-effort path, else prompt -------------------------
function onDrop(e: DragEvent) {
  dragDepth.value = 0;
  const dt = e.dataTransfer;
  if (!dt) return;
  clearMessages();
  const gen = ++dropGen;
  const p = pathFromDrop(dt);
  if (p) {
    input.value = p;
    void submit();
    return;
  }
  // No OS path (e.g. Chrome from Explorer): read the drop client-side to
  // confirm what it is, then ask the user to pin the location.
  void readDropped(dt, gen);
}

async function readDropped(dt: DataTransfer, gen: number) {
  try {
    const entry = dt.items?.[0]?.webkitGetAsEntry?.();
    const file = dt.files?.[0];
    if (entry?.isFile && entry.name.toLowerCase().endsWith(".devwebui")) {
      promptForLocation(gen, await projectNameFromFile(file), entry.name, false);
    } else if (entry?.isDirectory) {
      const name = await findProjectNameInDir(entry as FileSystemDirectoryEntry);
      promptForLocation(gen, name, entry.name, true);
    } else if (file?.name.toLowerCase().endsWith(".devwebui")) {
      promptForLocation(gen, await projectNameFromFile(file), file.name, false);
    } else if (gen === dropGen) {
      note.value = t("addProject.errDropRead");
    }
  } catch {
    if (gen === dropGen) note.value = t("addProject.errDropRead");
  }
}

function promptForLocation(
  gen: number,
  projectName: string | null,
  droppedName: string,
  isFolder: boolean,
) {
  if (gen !== dropGen) return; // a newer drop superseded this one
  const what = projectName
    ? t("addProject.dropNamed", { name: projectName })
    : isFolder
      ? t("addProject.dropFolder", { name: droppedName })
      : droppedName;
  note.value = t("addProject.dropPrompt", { what });
}
</script>

<template>
  <Dialog :open="open" @update:open="(v: boolean) => { if (!busy) open = v }">
    <DialogContent class="max-h-[90vh] overflow-y-auto overflow-x-hidden sm:max-w-135" :aria-busy="busy">
      <!-- One wrapper so DialogContent's own grid gap never applies: the header/footer
           margins below set this dialog's spacing. -->
      <div class="flex min-w-0 flex-col">
      <DialogHeader class="mb-4">
        <DialogTitle>
          {{ takeover ? t("addProject.titleTakeover") : t("addProject.titleAdd") }}
        </DialogTitle>
        <DialogDescription>
          {{ takeover ? t("addProject.descTakeover") : t("addProject.descAdd") }}
        </DialogDescription>
      </DialogHeader>
      <span aria-live="polite" class="sr-only">{{ busy ? t("addProject.working") : "" }}</span>

      <!-- Take over — the added repo also auto-starts its dev server outside DevWebUI -->
      <TakeoverStep
        v-if="takeover"
        v-model:open="open"
        v-model:takeover="takeover"
        v-model:busy="busy"
        v-model:error="error"
      />

      <div
        v-else
        class="flex min-w-0 flex-col gap-4"
        @dragover.prevent
        @dragenter.prevent="dragDepth++"
        @dragleave.prevent="dragDepth = Math.max(0, dragDepth - 1)"
        @drop.prevent="onDrop"
      >
        <!-- Manual add: drop zone, alerts, scaffold offer, paste path/URL, clone destination -->
        <AddManualForm
          v-model:scaffold="scaffold"
          v-model:input="input"
          v-model:dest="dest"
          :dragging="dragging"
          :busy="busy"
          :error="error"
          :note="note"
          :is-git-url="isGitUrl"
          @submit="submit"
          @clear-messages="clearMessages"
          @pick-dest="pickDest"
          @create-scaffold="createScaffold"
        />

        <!-- Scan ONE specific folder (explicit only — no machine-wide scan), and its results -->
        <ScanResultsPanel
          v-model:scan-folder="scanFolder"
          :busy="busy"
          :scanning="scanning"
          :scanned-folder="scannedFolder"
          :scan-result="scanResult"
          :ignored-paths="store.ignoredProjects"
          @browse-scan-folder="browseScanFolder"
          @scan-typed-folder="scanTypedFolder"
          @select="addFound"
          @ignore="ignoreDetected"
          @unignore="unignoreDetected"
        />
      </div>

      <!-- No explicit Close button: the corner ✕ and click-outside both dismiss. -->
      <DialogFooter v-if="!takeover" class="mt-4 sm:justify-start">
        <Hint :label="t('addProject.browseHint')" side="top">
          <Button variant="ghost" :disabled="busy" @click="browseFile">
            <FolderSearch class="size-4" /> {{ t("addProject.browseFile") }}
          </Button>
        </Hint>
      </DialogFooter>
      </div>
    </DialogContent>
  </Dialog>
</template>
