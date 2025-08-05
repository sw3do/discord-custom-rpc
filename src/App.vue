<script setup lang="ts">
import { ref, reactive, computed, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

interface ActivityData {
  details?: string;
  state?: string;
  activity_type?: number;
  timestamps?: {
    start?: number;
    end?: number;
  };
  assets?: {
    large_image?: string;
    large_text?: string;
    small_image?: string;
    small_text?: string;
  };
  buttons?: Array<{
    label: string;
    url: string;
  }>;
}

const currentLanguage = ref('en');
const isConnected = ref(false);
const connectionStatus = ref('');
const clientId = ref('');

const activity = reactive<ActivityData>({
  details: '',
  state: '',
  activity_type: 0,
  timestamps: {
    start: undefined,
    end: undefined
  },
  assets: {
    large_image: '',
    large_text: '',
    small_image: '',
    small_text: ''
  },
  buttons: []
});

const translations = {
  en: {
    title: 'Discord Custom RPC',
    subtitle: 'Create and manage your custom Discord Rich Presence',
    connection: 'Connection',
    clientId: 'Client ID',
    clientIdPlaceholder: 'Enter your Discord application Client ID',
    connect: 'Connect',
    connected: 'Connected',
    disconnect: 'Disconnect',
    activity: 'Activity Settings',
    details: 'Details',
    detailsPlaceholder: 'What are you doing?',
    state: 'State',
    statePlaceholder: 'Additional info',
    largeImage: 'Large Image',
    largeImagePlaceholder: 'Image key or URL',
    largeText: 'Large Image Text',
    largeTextPlaceholder: 'Text when hovering large image',
    smallImage: 'Small Image',
    smallImagePlaceholder: 'Small image key or URL',
    smallText: 'Small Image Text',
    smallTextPlaceholder: 'Text when hovering small image',
    startTime: 'Start Time',
    endTime: 'End Time',
    now: 'Now',
    buttons: 'Buttons',
    addButton: 'Add Button',
    removeButton: 'Remove',
    buttonLabel: 'Button Label',
    buttonLabelPlaceholder: 'Button text',
    buttonUrl: 'Button URL',
    buttonUrlPlaceholder: 'https://example.com',
    setActivity: 'Set Activity',
    clearActivity: 'Clear Activity',
    status: 'Status',
    language: 'Language'
  },
  tr: {
    title: 'Discord Özel RPC',
    subtitle: 'Özel Discord Rich Presence oluşturun ve yönetin',
    connection: 'Bağlantı',
    clientId: 'İstemci ID',
    clientIdPlaceholder: 'Discord uygulama İstemci ID\'nizi girin',
    connect: 'Bağlan',
    connected: 'Bağlandı',
    disconnect: 'Bağlantıyı Kes',
    activity: 'Aktivite Ayarları',
    details: 'Detaylar',
    detailsPlaceholder: 'Ne yapıyorsunuz?',
    state: 'Durum',
    statePlaceholder: 'Ek bilgi',
    largeImage: 'Büyük Resim',
    largeImagePlaceholder: 'Resim anahtarı veya URL',
    largeText: 'Büyük Resim Metni',
    largeTextPlaceholder: 'Büyük resmin üzerine gelindiğinde gösterilecek metin',
    smallImage: 'Küçük Resim',
    smallImagePlaceholder: 'Küçük resim anahtarı veya URL',
    smallText: 'Küçük Resim Metni',
    smallTextPlaceholder: 'Küçük resmin üzerine gelindiğinde gösterilecek metin',
    startTime: 'Başlangıç Zamanı',
    endTime: 'Bitiş Zamanı',
    now: 'Şimdi',
    buttons: 'Butonlar',
    addButton: 'Buton Ekle',
    removeButton: 'Kaldır',
    buttonLabel: 'Buton Etiketi',
    buttonLabelPlaceholder: 'Buton metni',
    buttonUrl: 'Buton URL',
    buttonUrlPlaceholder: 'https://ornek.com',
    setActivity: 'Aktiviteyi Ayarla',
    clearActivity: 'Aktiviteyi Temizle',
    status: 'Durum',
    language: 'Dil'
  },
  de: {
    title: 'Discord Benutzerdefinierte RPC',
    subtitle: 'Erstellen und verwalten Sie Ihre benutzerdefinierte Discord Rich Presence',
    connection: 'Verbindung',
    clientId: 'Client-ID',
    clientIdPlaceholder: 'Geben Sie Ihre Discord-Anwendungs-Client-ID ein',
    connect: 'Verbinden',
    connected: 'Verbunden',
    disconnect: 'Trennen',
    activity: 'Aktivitätseinstellungen',
    details: 'Details',
    detailsPlaceholder: 'Was machst du?',
    state: 'Status',
    statePlaceholder: 'Zusätzliche Informationen',
    largeImage: 'Großes Bild',
    largeImagePlaceholder: 'Bildschlüssel oder URL',
    largeText: 'Großer Bildtext',
    largeTextPlaceholder: 'Text beim Hovern über das große Bild',
    smallImage: 'Kleines Bild',
    smallImagePlaceholder: 'Kleiner Bildschlüssel oder URL',
    smallText: 'Kleiner Bildtext',
    smallTextPlaceholder: 'Text beim Hovern über das kleine Bild',
    startTime: 'Startzeit',
    endTime: 'Endzeit',
    now: 'Jetzt',
    buttons: 'Schaltflächen',
    addButton: 'Schaltfläche hinzufügen',
    removeButton: 'Entfernen',
    buttonLabel: 'Schaltflächenbeschriftung',
    buttonLabelPlaceholder: 'Schaltflächentext',
    buttonUrl: 'Schaltflächen-URL',
    buttonUrlPlaceholder: 'https://beispiel.com',
    setActivity: 'Aktivität festlegen',
    clearActivity: 'Aktivität löschen',
    status: 'Status',
    language: 'Sprache'
  }
};

const t = computed(() => translations[currentLanguage.value as keyof typeof translations]);

const isConnecting = ref(false);

async function connectToDiscord() {
  if (!clientId.value.trim()) {
    connectionStatus.value = 'Please enter a Client ID';
    return;
  }
  
  try {
    isConnecting.value = true;
    connectionStatus.value = 'Connecting to Discord...';
    const result = await invoke('connect_discord_rpc', { clientId: clientId.value });
    connectionStatus.value = result as string;
    isConnected.value = true;
    console.log('Connected successfully:', result);
  } catch (error) {
    connectionStatus.value = `Connection failed: ${error}`;
    isConnected.value = false;
    console.error('Connection failed:', error);
  } finally {
    isConnecting.value = false;
  }
}

async function disconnectFromDiscord() {
  try {
    const result = await invoke('disconnect_discord_rpc');
    isConnected.value = false;
    connectionStatus.value = result as string;
    console.log('Disconnected successfully:', result);
  } catch (error) {
    connectionStatus.value = `Disconnect failed: ${error}`;
    console.error('Disconnect failed:', error);
  }
}

async function checkConnectionStatus() {
  try {
    const connected = await invoke('get_connection_status');
    isConnected.value = connected as boolean;
    if (!connected) {
      connectionStatus.value = 'Not connected to Discord RPC';
    }
  } catch (error) {
    console.error('Failed to check connection status:', error);
    isConnected.value = false;
  }
}

async function setActivity() {
  if (!isConnected.value) {
    connectionStatus.value = 'Not connected to Discord';
    return;
  }
  
  try {
    const activityData: ActivityData = {
      details: activity.details || undefined,
      state: activity.state || undefined,
      activity_type: activity.activity_type,
      timestamps: {
        start: activity.timestamps?.start || undefined,
        end: activity.timestamps?.end || undefined
      },
      assets: {
        large_image: activity.assets?.large_image || undefined,
        large_text: activity.assets?.large_text || undefined,
        small_image: activity.assets?.small_image || undefined,
        small_text: activity.assets?.small_text || undefined
      },
      buttons: activity.buttons?.filter(btn => btn.label && btn.url) || []
    };
    
    console.log('Setting activity with data:', activityData);
    const result = await invoke('set_activity', { activity: activityData });
    connectionStatus.value = result as string;
    console.log('Activity set successfully:', result);
  } catch (error) {
    connectionStatus.value = `Failed to set activity: ${error}`;
    console.error('Failed to set activity:', error);
    
    await checkConnectionStatus();
  }
}

async function clearActivity() {
  if (!isConnected.value) {
    connectionStatus.value = 'Not connected to Discord RPC';
    return;
  }
  
  try {
    console.log('Clearing activity...');
    const result = await invoke('clear_activity');
    connectionStatus.value = result as string;
    console.log('Activity cleared successfully:', result);
  } catch (error) {
    connectionStatus.value = `Failed to clear activity: ${error}`;
    console.error('Failed to clear activity:', error);
    
    await checkConnectionStatus();
  }
}

function addButton() {
  if (!activity.buttons) {
    activity.buttons = [];
  }
  activity.buttons.push({ label: '', url: '' });
}

function removeButton(index: number) {
  if (activity.buttons) {
    activity.buttons.splice(index, 1);
  }
}

function setStartTimeNow() {
  if (!activity.timestamps) {
    activity.timestamps = {};
  }
  activity.timestamps.start = Math.floor(Date.now() / 1000);
}

function setEndTimeNow() {
  if (!activity.timestamps) {
    activity.timestamps = {};
  }
  activity.timestamps.end = Math.floor(Date.now() / 1000);
}

onMounted(() => {
  setInterval(async () => {
    if (isConnected.value) {
      await checkConnectionStatus();
    }
  }, 5000);
});
</script>

<template>
  <div class="min-h-screen bg-gray-900 text-white">
    <div class="container mx-auto px-4 py-8">
      <div class="flex justify-between items-center mb-8">
        <div>
          <h1 class="text-4xl font-bold text-indigo-400 mb-2">{{ t.title }}</h1>
          <p class="text-gray-400">{{ t.subtitle }}</p>
        </div>
        <div class="flex items-center space-x-4">
          <label class="text-sm text-gray-400">{{ t.language }}:</label>
          <select v-model="currentLanguage" class="bg-gray-800 border border-gray-700 rounded px-3 py-1 text-white focus:outline-none focus:border-indigo-500">
            <option value="en">English</option>
            <option value="tr">Türkçe</option>
            <option value="de">Deutsch</option>
          </select>
        </div>
      </div>

      <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
        <div class="bg-gray-800 rounded-lg p-6 border border-gray-700">
          <h2 class="text-xl font-semibold mb-4 text-indigo-300">Connection</h2>
          
          <div class="space-y-4">
            <div>
              <label class="block text-sm font-medium text-gray-300 mb-2">{{ t.clientId }}</label>
              <input 
                v-model="clientId" 
                type="text" 
                :placeholder="t.clientIdPlaceholder"
                class="w-full bg-gray-700 border border-gray-600 rounded px-3 py-2 text-white placeholder-gray-400 focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500"
              />
            </div>
            
            <div class="flex gap-2">
              <button 
                @click="connectToDiscord" 
                :disabled="!clientId.trim() || isConnected || isConnecting"
                class="flex-1 bg-indigo-600 hover:bg-indigo-700 disabled:bg-gray-600 disabled:cursor-not-allowed text-white font-medium py-2 px-4 rounded transition-colors"
              >
                {{ isConnecting ? 'Connecting...' : t.connect }}
              </button>
              
              <button 
                @click="disconnectFromDiscord" 
                :disabled="!isConnected"
                class="flex-1 bg-red-600 hover:bg-red-700 disabled:bg-gray-600 disabled:cursor-not-allowed text-white font-medium py-2 px-4 rounded transition-colors"
              >
                {{ t.disconnect }}
              </button>
            </div>
            
            <div v-if="connectionStatus" class="p-3 rounded" :class="isConnected ? 'bg-green-900 text-green-300' : 'bg-red-900 text-red-300'">
              <p class="text-sm">{{ t.status }}: {{ connectionStatus }}</p>
            </div>
          </div>
        </div>

        <div class="bg-gray-800 rounded-lg p-6 border border-gray-700">
          <h2 class="text-xl font-semibold mb-4 text-indigo-300">Activity Settings</h2>
          
          <div class="space-y-4">
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <label class="block text-sm font-medium text-gray-300 mb-2">{{ t.details }}</label>
                <input 
                  v-model="activity.details" 
                  type="text" 
                  :placeholder="t.detailsPlaceholder"
                  class="w-full bg-gray-700 border border-gray-600 rounded px-3 py-2 text-white placeholder-gray-400 focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500"
                />
              </div>
              
              <div>
                <label class="block text-sm font-medium text-gray-300 mb-2">{{ t.state }}</label>
                <input 
                  v-model="activity.state" 
                  type="text" 
                  :placeholder="t.statePlaceholder"
                  class="w-full bg-gray-700 border border-gray-600 rounded px-3 py-2 text-white placeholder-gray-400 focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500"
                />
              </div>
            </div>
            
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <label class="block text-sm font-medium text-gray-300 mb-2">{{ t.largeImage }}</label>
                <input 
                  v-model="activity.assets!.large_image" 
                  type="text" 
                  :placeholder="t.largeImagePlaceholder"
                  class="w-full bg-gray-700 border border-gray-600 rounded px-3 py-2 text-white placeholder-gray-400 focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500"
                />
              </div>
              
              <div>
                <label class="block text-sm font-medium text-gray-300 mb-2">{{ t.largeText }}</label>
                <input 
                  v-model="activity.assets!.large_text" 
                  type="text" 
                  :placeholder="t.largeTextPlaceholder"
                  class="w-full bg-gray-700 border border-gray-600 rounded px-3 py-2 text-white placeholder-gray-400 focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500"
                />
              </div>
            </div>
            
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <label class="block text-sm font-medium text-gray-300 mb-2">{{ t.smallImage }}</label>
                <input 
                  v-model="activity.assets!.small_image" 
                  type="text" 
                  :placeholder="t.smallImagePlaceholder"
                  class="w-full bg-gray-700 border border-gray-600 rounded px-3 py-2 text-white placeholder-gray-400 focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500"
                />
              </div>
              
              <div>
                <label class="block text-sm font-medium text-gray-300 mb-2">{{ t.smallText }}</label>
                <input 
                  v-model="activity.assets!.small_text" 
                  type="text" 
                  :placeholder="t.smallTextPlaceholder"
                  class="w-full bg-gray-700 border border-gray-600 rounded px-3 py-2 text-white placeholder-gray-400 focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500"
                />
              </div>
            </div>
            
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <label class="block text-sm font-medium text-gray-300 mb-2">{{ t.startTime }}</label>
                <div class="flex space-x-2">
                  <input 
                    v-model.number="activity.timestamps!.start" 
                    type="number" 
                    class="flex-1 bg-gray-700 border border-gray-600 rounded px-3 py-2 text-white placeholder-gray-400 focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500"
                  />
                  <button 
                    @click="setStartTimeNow" 
                    type="button"
                    class="bg-gray-600 hover:bg-gray-500 text-white px-3 py-2 rounded transition-colors"
                  >
                    Now
                  </button>
                </div>
              </div>
              
              <div>
                <label class="block text-sm font-medium text-gray-300 mb-2">{{ t.endTime }}</label>
                <div class="flex space-x-2">
                  <input 
                    v-model.number="activity.timestamps!.end" 
                    type="number" 
                    class="flex-1 bg-gray-700 border border-gray-600 rounded px-3 py-2 text-white placeholder-gray-400 focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500"
                  />
                  <button 
                    @click="setEndTimeNow" 
                    type="button"
                    class="bg-gray-600 hover:bg-gray-500 text-white px-3 py-2 rounded transition-colors"
                  >
                    Now
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
      
      <div class="mt-8 bg-gray-800 rounded-lg p-6 border border-gray-700">
        <div class="flex justify-between items-center mb-4">
          <h2 class="text-xl font-semibold text-indigo-300">Buttons</h2>
          <button 
            @click="addButton" 
            class="bg-green-600 hover:bg-green-700 text-white px-4 py-2 rounded transition-colors"
          >
            {{ t.addButton }}
          </button>
        </div>
        
        <div v-if="activity.buttons && activity.buttons.length > 0" class="space-y-3">
          <div v-for="(button, index) in activity.buttons" :key="index" class="flex space-x-3 items-end">
            <div class="flex-1">
              <label class="block text-sm font-medium text-gray-300 mb-1">{{ t.buttonLabel }}</label>
              <input 
                v-model="button.label" 
                type="text" 
                class="w-full bg-gray-700 border border-gray-600 rounded px-3 py-2 text-white placeholder-gray-400 focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500"
              />
            </div>
            <div class="flex-1">
              <label class="block text-sm font-medium text-gray-300 mb-1">{{ t.buttonUrl }}</label>
              <input 
                v-model="button.url" 
                type="url" 
                class="w-full bg-gray-700 border border-gray-600 rounded px-3 py-2 text-white placeholder-gray-400 focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500"
              />
            </div>
            <button 
              @click="removeButton(index)" 
              class="bg-red-600 hover:bg-red-700 text-white px-3 py-2 rounded transition-colors"
            >
              {{ t.removeButton }}
            </button>
          </div>
        </div>
      </div>
      
      <div class="mt-8 flex space-x-4">
        <button 
          @click="setActivity" 
          :disabled="!isConnected"
          class="flex-1 bg-green-600 hover:bg-green-700 disabled:bg-gray-600 disabled:cursor-not-allowed text-white font-medium py-3 px-6 rounded transition-colors"
        >
          {{ t.setActivity }}
        </button>
        
        <button 
          @click="clearActivity" 
          :disabled="!isConnected"
          class="flex-1 bg-red-600 hover:bg-red-700 disabled:bg-gray-600 disabled:cursor-not-allowed text-white font-medium py-3 px-6 rounded transition-colors"
        >
          {{ t.clearActivity }}
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
</style>