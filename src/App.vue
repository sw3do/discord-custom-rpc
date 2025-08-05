<script setup lang="ts">
import { ref, reactive, computed, onMounted, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useToast } from "vue-toastification";

const toast = useToast();

interface ActivityData {
  details?: string;
  state?: string;
  timestamps: {
    start?: number;
    end?: number;
  };
  assets: {
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
const isConnecting = ref(false);
const isUploading = ref(false);
const uploadProgress = ref('');
const showAdvanced = ref(false);

const activity = reactive<ActivityData>({
  details: '',
  state: '',
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

const timeOption = ref('none');
const durationMinutes = ref(30);

const ensureAssets = () => {
  if (!activity.assets) {
    activity.assets = {
      large_image: '',
      large_text: '',
      small_image: '',
      small_text: ''
    };
  }
};

const ensureTimestamps = () => {
  if (!activity.timestamps) {
    activity.timestamps = {
      start: undefined,
      end: undefined
    };
  }
};

ensureAssets();
ensureTimestamps();

const translations = {
  en: {
    title: 'Discord Custom RPC',
    subtitle: 'Create and manage your custom Discord Rich Presence',
    connection: 'Connection',
    clientId: 'Application ID',
    clientIdPlaceholder: 'Enter your Discord application ID',
    connect: 'Connect',
    connected: 'Connected',
    disconnect: 'Disconnect',
    activity: 'Rich Presence',
    details: 'Details',
    detailsPlaceholder: 'Playing a game',
    state: 'State',
    statePlaceholder: 'In a match',
    largeImage: 'Large Image',
    largeImagePlaceholder: 'Image key or URL',
    uploadLargeImage: 'Upload Image',
    largeText: 'Large Image Text',
    largeTextPlaceholder: 'Hover text for large image',
    smallImage: 'Small Image',
    smallImagePlaceholder: 'Small image key or URL',
    uploadSmallImage: 'Upload Small Image',
    smallText: 'Small Image Text',
    smallTextPlaceholder: 'Hover text for small image',
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
    setActivity: 'Update Presence',
    clearActivity: 'Clear Presence',
    status: 'Status',
    language: 'Language',
    advanced: 'Advanced Settings',
    showAdvanced: 'Show Advanced',
    hideAdvanced: 'Hide Advanced',
    timeDisplay: 'Time Display',
    noTime: 'No time display',
    elapsedTime: 'Show elapsed time (since now)',
    remainingTime: 'Show remaining time',
    duration: 'Duration (minutes)',
    minutes: 'minutes'
  },
  tr: {
    title: 'Discord Özel RPC',
    subtitle: 'Özel Discord Rich Presence oluşturun ve yönetin',
    connection: 'Bağlantı',
    clientId: 'Uygulama ID',
    clientIdPlaceholder: 'Discord uygulama ID\'nizi girin',
    connect: 'Bağlan',
    connected: 'Bağlandı',
    disconnect: 'Bağlantıyı Kes',
    activity: 'Rich Presence',
    details: 'Detaylar',
    detailsPlaceholder: 'Oyun oynuyor',
    state: 'Durum',
    statePlaceholder: 'Maçta',
    largeImage: 'Büyük Resim',
    largeImagePlaceholder: 'Resim anahtarı veya URL',
    uploadLargeImage: 'Resim Yükle',
    largeText: 'Büyük Resim Metni',
    largeTextPlaceholder: 'Büyük resim için hover metni',
    smallImage: 'Küçük Resim',
    smallImagePlaceholder: 'Küçük resim anahtarı veya URL',
    uploadSmallImage: 'Küçük Resim Yükle',
    smallText: 'Küçük Resim Metni',
    smallTextPlaceholder: 'Küçük resim için hover metni',
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
    setActivity: 'Presence Güncelle',
    clearActivity: 'Presence Temizle',
    status: 'Durum',
    language: 'Dil',
    advanced: 'Gelişmiş Ayarlar',
    showAdvanced: 'Gelişmiş Göster',
    hideAdvanced: 'Gelişmiş Gizle',
    timeDisplay: 'Zaman Gösterimi',
    noTime: 'Zaman gösterme',
    elapsedTime: 'Geçen zamanı göster (şu andan itibaren)',
    remainingTime: 'Kalan zamanı göster',
    duration: 'Süre (dakika)',
    minutes: 'dakika'
  }
};

const t = computed(() => translations[currentLanguage.value as keyof typeof translations]);

const uploadImage = async (file: File, imageType: 'large' | 'small') => {
  if (!file) return;
  
  isUploading.value = true;
  uploadProgress.value = 'Resizing image...';
  
  try {
    const reader = new FileReader();
    reader.onload = async (e) => {
      try {
        const base64Data = e.target?.result as string;
        
        uploadProgress.value = 'Uploading to server...';
        const imageUrl = await invoke<string>('upload_image_to_api', {
          imageData: base64Data
        });
        
        if (imageType === 'large') {
          activity.assets.large_image = imageUrl;
        } else {
          activity.assets.small_image = imageUrl;
        }
        
        toast.success(`${imageType === 'large' ? 'Large' : 'Small'} image uploaded successfully!`);
        uploadProgress.value = '';
        isUploading.value = false;
      } catch (error) {
        console.error('Upload failed:', error);
        toast.error(`Upload failed: ${error}`);
        uploadProgress.value = '';
        isUploading.value = false;
      }
    };
    reader.readAsDataURL(file);
  } catch (error) {
    console.error('File read failed:', error);
    toast.error('File read failed');
    uploadProgress.value = '';
    isUploading.value = false;
  }
};

const handleImageUpload = (event: Event, imageType: 'large' | 'small') => {
  const target = event.target as HTMLInputElement;
  const file = target.files?.[0];
  if (file) {
    uploadImage(file, imageType);
  }
};

async function connectToDiscord() {
  if (!clientId.value.trim()) {
    toast.error('Please enter an Application ID');
    return;
  }
  
  try {
    isConnecting.value = true;
    connectionStatus.value = 'Connecting to Discord...';
    const result = await invoke('connect_discord_rpc', { clientId: clientId.value });
    connectionStatus.value = result as string;
    isConnected.value = true;
    toast.success('Successfully connected to Discord!');
    console.log('Connected successfully:', result);
  } catch (error) {
    connectionStatus.value = `Connection failed: ${error}`;
    isConnected.value = false;
    toast.error(`Connection failed: ${error}`);
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
    toast.info('Disconnected from Discord');
    console.log('Disconnected successfully:', result);
  } catch (error) {
    connectionStatus.value = `Disconnect failed: ${error}`;
    toast.error(`Disconnect failed: ${error}`);
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
    toast.error('Not connected to Discord');
    return;
  }
  
  try {
    updateTimestamps();
    
    const activityData: ActivityData = {
      details: activity.details || undefined,
      state: activity.state || undefined,
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
    toast.success('Rich Presence updated successfully!');
    console.log('Activity set successfully:', result);
  } catch (error) {
    connectionStatus.value = `Failed to set activity: ${error}`;
    toast.error(`Failed to update presence: ${error}`);
    console.error('Failed to set activity:', error);
    
    await checkConnectionStatus();
  }
}

async function clearActivity() {
  if (!isConnected.value) {
    toast.error('Not connected to Discord RPC');
    return;
  }
  
  try {
    console.log('Clearing activity...');
    const result = await invoke('clear_activity');
    connectionStatus.value = result as string;
    toast.success('Rich Presence cleared successfully!');
    console.log('Activity cleared successfully:', result);
  } catch (error) {
    connectionStatus.value = `Failed to clear activity: ${error}`;
    toast.error(`Failed to clear presence: ${error}`);
    console.error('Failed to clear activity:', error);
    
    await checkConnectionStatus();
  }
}

function addButton() {
  if (!activity.buttons) {
    activity.buttons = [];
  }
  if (activity.buttons.length < 2) {
    activity.buttons.push({ label: '', url: '' });
    toast.info('Button added');
  } else {
    toast.warning('Maximum 2 buttons allowed');
  }
}

function removeButton(index: number) {
  if (activity.buttons) {
    activity.buttons.splice(index, 1);
    toast.info('Button removed');
  }
}

const updateTimestamps = () => {
  ensureTimestamps();
  const now = Math.floor(Date.now() / 1000);
  
  switch (timeOption.value) {
    case 'none':
      activity.timestamps.start = undefined;
      activity.timestamps.end = undefined;
      break;
    case 'elapsed':
      activity.timestamps.start = now;
      activity.timestamps.end = undefined;
      break;
    case 'remaining':
      activity.timestamps.start = undefined;
      activity.timestamps.end = now + (durationMinutes.value * 60);
      break;
  }
};

watch([timeOption, durationMinutes], updateTimestamps);



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
    <div class="bg-gray-800 border-b border-gray-700 px-4 sm:px-6 py-4">
      <div class="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4">
        <div class="flex items-center space-x-3 sm:space-x-4">
          <div class="w-8 h-8 bg-indigo-600 rounded-lg flex items-center justify-center flex-shrink-0">
            <svg class="w-5 h-5 text-white" fill="currentColor" viewBox="0 0 20 20">
              <path d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/>
            </svg>
          </div>
          <div class="min-w-0">
            <h1 class="text-lg sm:text-xl font-semibold text-white truncate">{{ t.title }}</h1>
            <p class="text-xs sm:text-sm text-gray-400 truncate">{{ t.subtitle }}</p>
          </div>
        </div>
        <div class="flex flex-col sm:flex-row items-start sm:items-center gap-3 sm:gap-4 w-full sm:w-auto">
          <div class="flex items-center space-x-2">
            <span class="text-sm text-gray-400 whitespace-nowrap">{{ t.language }}:</span>
            <select v-model="currentLanguage" class="bg-gray-700 border border-gray-600 rounded-md px-2 sm:px-3 py-1 text-sm text-white focus:outline-none focus:ring-2 focus:ring-indigo-500">
              <option value="en">English</option>
              <option value="tr">Türkçe</option>
            </select>
          </div>
          <div class="flex items-center space-x-2">
            <div class="w-3 h-3 rounded-full flex-shrink-0" :class="isConnected ? 'bg-green-500' : 'bg-red-500'"></div>
            <span class="text-sm whitespace-nowrap" :class="isConnected ? 'text-green-400' : 'text-red-400'">
              {{ isConnected ? t.connected : 'Disconnected' }}
            </span>
          </div>
        </div>
      </div>
    </div>

    <div class="flex flex-col lg:flex-row min-h-screen">
      <div class="w-full lg:w-80 xl:w-96 bg-gray-800 border-b lg:border-b-0 lg:border-r border-gray-700 p-4 sm:p-6">
        <div class="space-y-4 sm:space-y-6">
          <div class="bg-gray-750 rounded-lg p-4 border border-gray-600">
            <h2 class="text-lg font-semibold mb-4 text-indigo-300 flex items-center">
              <svg class="w-5 h-5 mr-2" fill="currentColor" viewBox="0 0 20 20">
                <path d="M3 4a1 1 0 011-1h12a1 1 0 011 1v2a1 1 0 01-1 1H4a1 1 0 01-1-1V4zM3 10a1 1 0 011-1h6a1 1 0 011 1v6a1 1 0 01-1 1H4a1 1 0 01-1-1v-6zM14 9a1 1 0 00-1 1v6a1 1 0 001 1h2a1 1 0 001-1v-6a1 1 0 00-1-1h-2z"/>
              </svg>
              {{ t.connection }}
            </h2>
            
            <div class="space-y-4">
              <div>
                <label class="block text-sm font-medium text-gray-300 mb-2">{{ t.clientId }}</label>
                <input 
                  v-model="clientId" 
                  type="text" 
                  :placeholder="t.clientIdPlaceholder"
                  class="w-full bg-gray-700 border border-gray-600 rounded-md px-3 py-2 text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
                />
              </div>
              
              <div class="flex flex-col sm:flex-row gap-2">
                <button 
                  @click="connectToDiscord" 
                  :disabled="!clientId.trim() || isConnected || isConnecting"
                  class="flex-1 bg-indigo-600 hover:bg-indigo-700 disabled:bg-gray-600 disabled:cursor-not-allowed text-white font-medium py-2 px-3 sm:px-4 rounded-md transition-colors flex items-center justify-center text-sm sm:text-base"
                >
                  <svg v-if="isConnecting" class="animate-spin -ml-1 mr-2 h-4 w-4 text-white" fill="none" viewBox="0 0 24 24">
                    <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                    <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                  </svg>
                  {{ isConnecting ? 'Connecting...' : t.connect }}
                </button>
                
                <button 
                  @click="disconnectFromDiscord" 
                  :disabled="!isConnected"
                  class="flex-1 bg-red-600 hover:bg-red-700 disabled:bg-gray-600 disabled:cursor-not-allowed text-white font-medium py-2 px-3 sm:px-4 rounded-md transition-colors text-sm sm:text-base"
                >
                  {{ t.disconnect }}
                </button>
              </div>
              
              <div v-if="connectionStatus" class="p-3 rounded-md text-sm" :class="isConnected ? 'bg-green-900/50 text-green-300 border border-green-700' : 'bg-red-900/50 text-red-300 border border-red-700'">
                {{ connectionStatus }}
              </div>
            </div>
          </div>

          <div class="bg-gray-750 rounded-lg p-4 border border-gray-600">
            <div class="flex items-center justify-between mb-4">
              <h3 class="text-lg font-semibold text-indigo-300 flex items-center">
                <svg class="w-5 h-5 mr-2" fill="currentColor" viewBox="0 0 20 20">
                  <path d="M13 6a3 3 0 11-6 0 3 3 0 016 0zM18 8a2 2 0 11-4 0 2 2 0 014 0zM14 15a4 4 0 00-8 0v3h8v-3z"/>
                </svg>
                {{ t.advanced }}
              </h3>
              <button 
                @click="showAdvanced = !showAdvanced"
                class="text-sm text-indigo-400 hover:text-indigo-300 transition-colors"
              >
                {{ showAdvanced ? t.hideAdvanced : t.showAdvanced }}
              </button>
            </div>
            
            <div v-if="showAdvanced" class="space-y-4">

              
              <div>
                <label class="block text-sm font-medium text-gray-300 mb-3">{{ t.timeDisplay }}</label>
                <div class="space-y-3">
                  <div class="flex items-center space-x-3">
                    <input 
                      type="radio" 
                      id="no-time" 
                      v-model="timeOption" 
                      value="none"
                      class="w-4 h-4 text-indigo-600 bg-gray-700 border-gray-600 focus:ring-indigo-500"
                    />
                    <label for="no-time" class="text-sm text-gray-300">{{ t.noTime }}</label>
                  </div>
                  
                  <div class="flex items-center space-x-3">
                    <input 
                      type="radio" 
                      id="elapsed-time" 
                      v-model="timeOption" 
                      value="elapsed"
                      class="w-4 h-4 text-indigo-600 bg-gray-700 border-gray-600 focus:ring-indigo-500"
                    />
                    <label for="elapsed-time" class="text-sm text-gray-300">{{ t.elapsedTime }}</label>
                  </div>
                  
                  <div class="flex items-center space-x-3">
                    <input 
                      type="radio" 
                      id="remaining-time" 
                      v-model="timeOption" 
                      value="remaining"
                      class="w-4 h-4 text-indigo-600 bg-gray-700 border-gray-600 focus:ring-indigo-500"
                    />
                    <label for="remaining-time" class="text-sm text-gray-300">{{ t.remainingTime }}</label>
                  </div>
                  
                  <div v-if="timeOption === 'remaining'" class="ml-7 mt-2">
                    <label class="block text-xs text-gray-400 mb-1">{{ t.duration }}</label>
                    <div class="flex space-x-2">
                      <input 
                        v-model.number="durationMinutes" 
                        type="number" 
                        min="1" 
                        max="1440" 
                        placeholder="30"
                        class="w-20 bg-gray-700 border border-gray-600 rounded-md px-2 py-1 text-white text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500"
                      />
                      <span class="text-sm text-gray-400 self-center">{{ t.minutes }}</span>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div class="flex-1 p-4 sm:p-6 overflow-y-auto">
        <div class="max-w-4xl mx-auto space-y-4 sm:space-y-6">
          <div class="bg-gray-800 rounded-lg p-4 sm:p-6 border border-gray-700">
            <h2 class="text-lg sm:text-xl font-semibold mb-4 sm:mb-6 text-indigo-300 flex items-center">
              <svg class="w-5 h-5 sm:w-6 sm:h-6 mr-2" fill="currentColor" viewBox="0 0 20 20">
                <path d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/>
              </svg>
              {{ t.activity }}
            </h2>
            
            <div class="grid grid-cols-1 lg:grid-cols-2 gap-4 sm:gap-6">
              <div>
                <label class="block text-sm font-medium text-gray-300 mb-2">{{ t.details }}</label>
                <input 
                  v-model="activity.details" 
                  type="text" 
                  :placeholder="t.detailsPlaceholder"
                  class="w-full bg-gray-700 border border-gray-600 rounded-md px-3 py-2 text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
                />
              </div>
              
              <div>
                <label class="block text-sm font-medium text-gray-300 mb-2">{{ t.state }}</label>
                <input 
                  v-model="activity.state" 
                  type="text" 
                  :placeholder="t.statePlaceholder"
                  class="w-full bg-gray-700 border border-gray-600 rounded-md px-3 py-2 text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
                />
              </div>
            </div>
          </div>

          <div class="bg-gray-800 rounded-lg p-4 sm:p-6 border border-gray-700">
            <h3 class="text-lg font-semibold mb-4 text-indigo-300 flex items-center">
              <svg class="w-5 h-5 mr-2" fill="currentColor" viewBox="0 0 20 20">
                <path d="M4 3a2 2 0 00-2 2v10a2 2 0 002 2h12a2 2 0 002-2V5a2 2 0 00-2-2H4zm12 12H4l4-8 3 6 2-4 3 6z"/>
              </svg>
              Images
            </h3>
            
            <div class="grid grid-cols-1 lg:grid-cols-2 gap-4 sm:gap-6">
              <div class="space-y-4">
                <div>
                  <label class="block text-sm font-medium text-gray-300 mb-2">{{ t.largeImage }}</label>
                  <div class="space-y-2">
                    <input 
                      v-model="activity.assets.large_image" 
                      type="text" 
                      :placeholder="t.largeImagePlaceholder"
                      class="w-full bg-gray-700 border border-gray-600 rounded-md px-3 py-2 text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
                    />
                    <div class="flex gap-2">
                      <input 
                        type="file" 
                        accept="image/*" 
                        @change="handleImageUpload($event, 'large')"
                        class="hidden"
                        id="large-image-upload"
                      />
                      <label 
                        for="large-image-upload" 
                        class="flex-1 bg-blue-600 hover:bg-blue-700 disabled:bg-gray-600 text-white text-xs sm:text-sm font-medium py-2 px-2 sm:px-3 rounded-md cursor-pointer text-center transition-colors flex items-center justify-center"
                        :class="{ 'opacity-50 cursor-not-allowed': isUploading }"
                      >
                        <svg class="w-3 h-3 sm:w-4 sm:h-4 mr-1 sm:mr-2" fill="currentColor" viewBox="0 0 20 20">
                          <path d="M5.5 13a3.5 3.5 0 01-.369-6.98 4 4 0 117.753-1.977A4.5 4.5 0 1113.5 13H11V9.413l1.293 1.293a1 1 0 001.414-1.414l-3-3a1 1 0 00-1.414 0l-3 3a1 1 0 001.414 1.414L9 9.413V13H5.5z"/>
                        </svg>
                        <span class="hidden sm:inline">{{ t.uploadLargeImage }}</span>
                        <span class="sm:hidden">Upload</span>
                      </label>
                    </div>
                  </div>
                </div>
                
                <div>
                  <label class="block text-sm font-medium text-gray-300 mb-2">{{ t.largeText }}</label>
                  <input 
                    v-model="activity.assets.large_text" 
                    type="text" 
                    :placeholder="t.largeTextPlaceholder"
                    class="w-full bg-gray-700 border border-gray-600 rounded-md px-3 py-2 text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
                  />
                </div>
              </div>
              
              <div class="space-y-4">
                <div>
                  <label class="block text-sm font-medium text-gray-300 mb-2">{{ t.smallImage }}</label>
                  <div class="space-y-2">
                    <input 
                      v-model="activity.assets.small_image" 
                      type="text" 
                      :placeholder="t.smallImagePlaceholder"
                      class="w-full bg-gray-700 border border-gray-600 rounded-md px-3 py-2 text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
                    />
                    <div class="flex gap-2">
                      <input 
                        type="file" 
                        accept="image/*" 
                        @change="handleImageUpload($event, 'small')"
                        class="hidden"
                        id="small-image-upload"
                      />
                      <label 
                        for="small-image-upload" 
                        class="flex-1 bg-blue-600 hover:bg-blue-700 disabled:bg-gray-600 text-white text-xs sm:text-sm font-medium py-2 px-2 sm:px-3 rounded-md cursor-pointer text-center transition-colors flex items-center justify-center"
                        :class="{ 'opacity-50 cursor-not-allowed': isUploading }"
                      >
                        <svg class="w-3 h-3 sm:w-4 sm:h-4 mr-1 sm:mr-2" fill="currentColor" viewBox="0 0 20 20">
                          <path d="M5.5 13a3.5 3.5 0 01-.369-6.98 4 4 0 117.753-1.977A4.5 4.5 0 1113.5 13H11V9.413l1.293 1.293a1 1 0 001.414-1.414l-3-3a1 1 0 00-1.414 0l-3 3a1 1 0 001.414 1.414L9 9.413V13H5.5z"/>
                        </svg>
                        <span class="hidden sm:inline">{{ t.uploadSmallImage }}</span>
                        <span class="sm:hidden">Upload</span>
                      </label>
                    </div>
                  </div>
                </div>
                
                <div>
                  <label class="block text-sm font-medium text-gray-300 mb-2">{{ t.smallText }}</label>
                  <input 
                    v-model="activity.assets.small_text" 
                    type="text" 
                    :placeholder="t.smallTextPlaceholder"
                    class="w-full bg-gray-700 border border-gray-600 rounded-md px-3 py-2 text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
                  />
                </div>
              </div>
            </div>
            
            <div v-if="isUploading" class="mt-4 p-4 bg-blue-900/50 border border-blue-700 rounded-lg">
              <div class="flex items-center space-x-3">
                <div class="animate-spin rounded-full h-5 w-5 border-b-2 border-blue-400"></div>
                <span class="text-blue-300">{{ uploadProgress || 'Uploading...' }}</span>
              </div>
            </div>
          </div>

          <div class="bg-gray-800 rounded-lg p-4 sm:p-6 border border-gray-700">
            <div class="flex flex-col sm:flex-row justify-between items-start sm:items-center mb-4 gap-3">
              <h3 class="text-lg font-semibold text-indigo-300 flex items-center">
                <svg class="w-5 h-5 mr-2" fill="currentColor" viewBox="0 0 20 20">
                  <path d="M8 9a3 3 0 100-6 3 3 0 000 6zM8 11a6 6 0 016 6H2a6 6 0 016-6zM16 7a1 1 0 10-2 0v1h-1a1 1 0 100 2h1v1a1 1 0 102 0v-1h1a1 1 0 100-2h-1V7z"/>
                </svg>
                {{ t.buttons }} ({{ activity.buttons?.length || 0 }}/2)
              </h3>
              <button 
                @click="addButton" 
                :disabled="(activity.buttons?.length || 0) >= 2"
                class="bg-green-600 hover:bg-green-700 disabled:bg-gray-600 disabled:cursor-not-allowed text-white px-3 sm:px-4 py-2 rounded-md transition-colors flex items-center text-sm sm:text-base w-full sm:w-auto justify-center"
              >
                <svg class="w-4 h-4 mr-2" fill="currentColor" viewBox="0 0 20 20">
                  <path d="M10 5a1 1 0 011 1v3h3a1 1 0 110 2h-3v3a1 1 0 11-2 0v-3H6a1 1 0 110-2h3V6a1 1 0 011-1z"/>
                </svg>
                {{ t.addButton }}
              </button>
            </div>
            
            <div v-if="activity.buttons && activity.buttons.length > 0" class="space-y-3">
              <div v-for="(button, index) in activity.buttons" :key="index" class="flex flex-col sm:flex-row space-y-3 sm:space-y-0 sm:space-x-3 items-start sm:items-end p-4 bg-gray-750 rounded-lg border border-gray-600">
                <div class="flex-1 w-full">
                  <label class="block text-sm font-medium text-gray-300 mb-2">{{ t.buttonLabel }}</label>
                  <input 
                    v-model="button.label" 
                    type="text" 
                    :placeholder="t.buttonLabelPlaceholder"
                    class="w-full bg-gray-700 border border-gray-600 rounded-md px-3 py-2 text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
                  />
                </div>
                <div class="flex-1 w-full">
                  <label class="block text-sm font-medium text-gray-300 mb-2">{{ t.buttonUrl }}</label>
                  <input 
                    v-model="button.url" 
                    type="url" 
                    :placeholder="t.buttonUrlPlaceholder"
                    class="w-full bg-gray-700 border border-gray-600 rounded-md px-3 py-2 text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
                  />
                </div>
                <button 
                  @click="removeButton(index)" 
                  class="bg-red-600 hover:bg-red-700 text-white px-3 py-2 rounded-md transition-colors flex items-center justify-center w-full sm:w-auto"
                >
                  <svg class="w-4 h-4 mr-2 sm:mr-0" fill="currentColor" viewBox="0 0 20 20">
                    <path d="M9 2a1 1 0 000 2h2a1 1 0 100-2H9z"/>
                    <path fill-rule="evenodd" d="M4 5a2 2 0 012-2h8a2 2 0 012 2v10a2 2 0 01-2 2H6a2 2 0 01-2-2V5zm3 4a1 1 0 112 0v4a1 1 0 11-2 0V9zm4 0a1 1 0 112 0v4a1 1 0 11-2 0V9z" clip-rule="evenodd"/>
                  </svg>
                  <span class="sm:hidden">{{ t.removeButton }}</span>
                </button>
              </div>
            </div>
            
            <div v-else class="text-center py-8 text-gray-400">
              <svg class="w-12 h-12 mx-auto mb-4 opacity-50" fill="currentColor" viewBox="0 0 20 20">
                <path d="M8 9a3 3 0 100-6 3 3 0 000 6zM8 11a6 6 0 016 6H2a6 6 0 016-6zM16 7a1 1 0 10-2 0v1h-1a1 1 0 100 2h1v1a1 1 0 102 0v-1h1a1 1 0 100-2h-1V7z"/>
              </svg>
              <p>No buttons added yet</p>
              <p class="text-sm">Click "Add Button" to create interactive buttons</p>
            </div>
          </div>

          <div class="flex flex-col sm:flex-row space-y-3 sm:space-y-0 sm:space-x-4">
            <button 
              @click="setActivity" 
              :disabled="!isConnected"
              class="flex-1 bg-green-600 hover:bg-green-700 disabled:bg-gray-600 disabled:cursor-not-allowed text-white font-medium py-3 px-4 sm:px-6 rounded-md transition-colors flex items-center justify-center text-sm sm:text-base"
            >
              <svg class="w-4 h-4 sm:w-5 sm:h-5 mr-2" fill="currentColor" viewBox="0 0 20 20">
                <path d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/>
              </svg>
              {{ t.setActivity }}
            </button>
            
            <button 
              @click="clearActivity" 
              :disabled="!isConnected"
              class="flex-1 bg-red-600 hover:bg-red-700 disabled:bg-gray-600 disabled:cursor-not-allowed text-white font-medium py-3 px-4 sm:px-6 rounded-md transition-colors flex items-center justify-center text-sm sm:text-base"
            >
              <svg class="w-4 h-4 sm:w-5 sm:h-5 mr-2" fill="currentColor" viewBox="0 0 20 20">
                <path d="M9 2a1 1 0 000 2h2a1 1 0 100-2H9z"/>
                <path fill-rule="evenodd" d="M4 5a2 2 0 012-2h8a2 2 0 012 2v10a2 2 0 01-2 2H6a2 2 0 01-2-2V5zm5 4a1 1 0 10-2 0v4a1 1 0 102 0V9zm4 0a1 1 0 10-2 0v4a1 1 0 102 0V9z" clip-rule="evenodd"/>
              </svg>
              {{ t.clearActivity }}
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.bg-gray-750 {
  background-color: #374151;
}
</style>