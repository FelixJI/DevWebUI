<script setup lang="ts">
import { ref } from "vue";
import { useI18n } from "vue-i18n";
import { Eye, EyeOff, FolderSearch, Search } from "@lucide/vue";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import ScanResults from "@/components/ScanResults.vue";
import type { ScanResult } from "@/api";

const { t } = useI18n({ useScope: "global" });

defineProps<{
  busy: boolean;
  scanning: boolean;
  scannedFolder?: string;
  scanResult: ScanResult | null;
  ignoredPaths: string[];
}>();
const emit = defineEmits<{
  browseScanFolder: [];
  scanTypedFolder: [];
  select: [path: string];
  ignore: [path: string];
  unignore: [path: string];
}>();

// Explicit "scan a specific folder" card — the ONLY scan this build offers. There is
// no machine-wide sweep, so scoping never depends on the user knowing a trick.
const scanFolder = defineModel<string>("scanFolder", { required: true });

// Reveal the folders the user dismissed from earlier folder scans (so they can un-ignore them).
const showIgnored = ref(false);
</script>

<template>
  <!-- Scan a specific folder — an explicit, discoverable action, visually set apart
       (tinted card, like the scaffold/take-over callouts) from the plain-text inputs. -->
  <div class="flex flex-col gap-1.5 rounded-xl border border-primary/30 bg-primary/5 p-3 text-sm">
    <div class="flex items-center gap-2 font-medium text-foreground">
      <FolderSearch class="size-4 text-primary" />
      {{ t("addProject.scanFolderHeading") }}
    </div>
    <p class="text-xs text-muted-foreground">{{ t("addProject.scanFolderBody") }}</p>
    <div class="mt-1 flex gap-2">
      <Input
        v-model="scanFolder"
        class="flex-1"
        :placeholder="t('addProject.scanFolderPlaceholder')"
        @keydown.enter="emit('scanTypedFolder')"
      />
      <Button :disabled="busy || scanning" @click="emit('browseScanFolder')">
        <Search v-if="scanning && scannedFolder" class="size-4 animate-pulse" />
        <FolderSearch v-else class="size-4" />
        {{ scanning && scannedFolder ? t("addProject.scanning") : t("addProject.browse") }}
      </Button>
    </div>
  </div>

  <!-- Results of the folder scan above. -->
  <p v-if="scannedFolder && scanResult" class="px-1 text-xs text-muted-foreground">
    {{ t("addProject.scannedFolderCaption", { folder: scannedFolder }) }}
  </p>
  <ScanResults
    v-if="scanResult"
    v-auto-animate
    compact
    :result="scanResult"
    :busy="busy"
    :ignored-paths="ignoredPaths"
    :show-ignored="showIgnored"
    @select="emit('select', $event)"
    @ignore="emit('ignore', $event)"
    @unignore="emit('unignore', $event)"
  />
  <Button
    v-if="ignoredPaths.length"
    variant="ghost"
    size="sm"
    :disabled="busy"
    class="self-start"
    @click="showIgnored = !showIgnored"
  >
    <Eye v-if="showIgnored" class="size-4" />
    <EyeOff v-else class="size-4" />
    {{ t("addProject.showIgnored") }}
  </Button>
</template>
