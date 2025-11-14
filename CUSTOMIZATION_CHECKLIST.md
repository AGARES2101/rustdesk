# 📋 Анкета кастомизации RustDesk

## 🎨 1. БРЕНДИНГ И НАЗВАНИЕ

### 1.1 Название приложения
- [ ] **Название приложения** (отображается в заголовке окна, трее)
  - Текущее: `Ruware Desk`
  - Файл: `flutter/lib/desktop/pages/desktop_home_page.dart`
  - Переменная workflow: `APP_NAME` в `.github/workflows/flutter-build.yml`

- [ ] **Название компании**
  - Текущее: `My Company`
  - Файл: `flutter/windows/runner/Runner.rc` (строка 92, 96)
  - Переменная workflow: `COMPANY_NAME`

- [ ] **Описание приложения**
  - Текущее: `Ruware Desk Remote Desktop`
  - Файл: `flutter/windows/runner/Runner.rc` (строка 93)

- [ ] **Copyright текст**
  - Текущее: `Copyright © 2025 My Company. All rights reserved.`
  - Файл: `flutter/windows/runner/Runner.rc` (строка 96)

### 1.2 URL и ссылки
- [ ] **Официальный сайт** (ссылка "powered by me")
  - Текущий: `https://rustdesk.com`
  - Файл: `flutter/lib/common.dart` (строка 3602)

- [ ] **URL схема** (для deep links)
  - Текущая: `rustdesk://`
  - Формируется автоматически из названия приложения

---

## 🎨 2. ЦВЕТОВАЯ СХЕМА

### 2.1 Основные цвета бренда
- [ ] **Акцентный цвет** (кнопки, ссылки, активные элементы)
  - Текущий: `#9C1D12` (красный Ruware Desk)
  - Файл: `flutter/lib/common.dart` (строка 254)
  - Переменная: `MyTheme.accent`

- [ ] **Цвет акцента 50% прозрачности**
  - Текущий: `#779C1D12`
  - Файл: `flutter/lib/common.dart` (строка 255)
  - Переменная: `MyTheme.accent50`

- [ ] **Цвет акцента 80% прозрачности**
  - Текущий: `#AA9C1D12`
  - Файл: `flutter/lib/common.dart` (строка 256)
  - Переменная: `MyTheme.accent80`

### 2.2 Дополнительные цвета
- [ ] **Цвет ID** (отображение ID подключения)
  - Текущий: `#00B6F0` (голубой)
  - Файл: `flutter/lib/common.dart` (строка 259)
  - Переменная: `MyTheme.idColor`

- [ ] **Цвет для Connection Manager**
  - Текущий: `#21790B` (зеленый)
  - Файл: `flutter/lib/common.dart` (строка 261)
  - Переменная: `MyTheme.cmIdColor`

- [ ] **Фоновый цвет (светлая тема)**
  - Текущий: `#EFEFF2` (серый)
  - Файл: `flutter/lib/common.dart` (строка 252)
  - Переменная: `MyTheme.grayBg`

- [ ] **Цвет границ**
  - Текущий: `#CCCCCC`
  - Файл: `flutter/lib/common.dart` (строка 258)
  - Переменная: `MyTheme.border`

---

## 🖼️ 3. ЛОГОТИПЫ И ИКОНКИ

### 3.1 Иконка приложения
- [ ] **Иконка Windows (.ico)**
  - Файл: `res/icon.ico`
  - Требования: 256x256, 128x128, 64x64, 48x48, 32x32, 16x16 пикселей
  - Формат: ICO (многослойный)

- [ ] **Иконка трея Windows**
  - Файл: `res/tray-icon.ico`
  - Требования: 16x16, 32x32 пикселей
  - Формат: ICO

- [ ] **PNG иконки (разные размеры)**
  - [ ] `res/32x32.png` - 32x32 px
  - [ ] `res/64x64.png` - 64x64 px
  - [ ] `res/128x128.png` - 128x128 px
  - [ ] `res/128x128@2x.png` - 256x256 px (Retina)
  - [ ] `res/icon.png` - основная PNG (рекомендуется 512x512 px)

### 3.2 Логотип приложения
- [ ] **Векторный логотип (SVG)**
  - Файл: `res/logo.svg`
  - Отображается: на главной странице
  - Рекомендации: простой дизайн, читаемый в малых размерах

- [ ] **Растровый логотип (PNG)**
  - Файл: `flutter/assets/logo.png`
  - Максимальный размер: 300x60 пикселей
  - Используется в Flutter UI

### 3.3 Иконки для других платформ
- [ ] **macOS иконка**
  - Файл: `flutter/macos/Runner/Assets.xcassets/AppIcon.appiconset/`
  - Требуется несколько размеров

- [ ] **Linux иконка**
  - Файл: `res/rustdesk.desktop` (путь к иконке)
  - Формат: PNG или SVG

- [ ] **Android иконка**
  - Путь: `flutter/android/app/src/main/res/mipmap-*/`
  - Требуется: hdpi, mdpi, xhdpi, xxhdpi, xxxhdpi

---

## 📱 4. UI ЭЛЕМЕНТЫ

### 4.1 Главное окно
- [ ] **Заголовок главного окна**
  - Файл: `flutter/lib/desktop/pages/desktop_home_page.dart`
  - Текст приветствия, инструкции

- [ ] **Текст "Powered by"**
  - Текущий: скрыт или "powered by RustDesk"
  - Файл: `flutter/lib/common.dart` (функция `loadPowered`)
  - Можно скрыть через: `hide-powered-by-me` в builtin options

### 4.2 Титульный бар (Windows)
- [ ] **Иконка в титульном баре**
  - Файл: `flutter/windows/runner/resources/app_icon.ico`
  - Используется автоматически из `Runner.rc`

- [ ] **Название в титульном баре**
  - Формируется из `APP_NAME`
  - Для Connection Manager: `{APP_NAME} - Connection Manager`
  - Для Install: `{APP_NAME} - Install`

### 4.3 Диалоги и уведомления
- [ ] **Цвет иконок информационных сообщений**
  - Используется `MyTheme.accent` по умолчанию
  - Файл: `flutter/lib/common.dart` (функция `_msgboxColor`)

- [ ] **Фон error баннера (светлая тема)**
  - Текущий: `#FDEEEB`
  - Файл: `flutter/lib/common.dart` (строка 177)

- [ ] **Фон error баннера (темная тема)**
  - Текущий: `#470F2D`
  - Файл: `flutter/lib/common.dart` (строка 191)

---

## 🌐 5. СЕРВЕР И НАСТРОЙКИ

### 5.1 Серверные настройки (в workflow)
- [ ] **Rendezvous Server** (основной сервер)
  - Текущий: `172.25.84.149`
  - Переменная: `RUSTDESK_SERVER`

- [ ] **API Server**
  - Текущий: `http://172.25.84.149`
  - Переменная: `RUSTDESK_API_SERVER`

- [ ] **Relay Server**
  - Текущий: `172.25.84.149`
  - Переменная: `RENDEZVOUS_SERVER`

- [ ] **Public Key**
  - Текущий: `VFp5iZemmI68dp03gd6UhOzeavhFmklTEEnKuQqItqY=`
  - Переменная: `RS_PUB_KEY`

### 5.2 Скрытие функционала
- [ ] **Скрыть настройки сервера**
  - Опция: `hide-server-settings`
  
- [ ] **Скрыть настройки прокси**
  - Опция: `hide-proxy-settings`

- [ ] **Скрыть настройки безопасности**
  - Опция: `hide-security-settings`

- [ ] **Скрыть "powered by me"**
  - Опция: `hide-powered-by-me`

---

## 📄 6. ТЕКСТОВЫЕ ФАЙЛЫ И МЕТАДАННЫЕ

### 6.1 Windows метаданные
- [ ] **FileDescription**
  - Текущий: `Ruware Desk Remote Desktop`
  - Файл: `flutter/windows/runner/Runner.rc` (строка 93)

- [ ] **ProductName**
  - Текущий: `Ruware Desk`
  - Файл: `flutter/windows/runner/Runner.rc` (строка 98)

- [ ] **InternalName**
  - Текущий: `rustdesk`
  - Файл: `flutter/windows/runner/Runner.rc` (строка 95)

### 6.2 Android метаданные
- [ ] **app_name (Android)**
  - Файл: `flutter/android/app/src/main/res/values/strings.xml`
  - Текущий: `Ruware Desk`

- [ ] **Package name (Android)**
  - Файл: `flutter/android/app/build.gradle`
  - Текущий: `com.carriez.flutter_hbb`

### 6.3 iOS/macOS метаданные
- [ ] **CFBundleDisplayName (iOS)**
  - Файл: `flutter/ios/Runner/Info.plist`
  - Текущий: `Ruware Desk`

- [ ] **Bundle Identifier (iOS)**
  - Файл: `flutter/ios/Runner/Info.plist`
  - Формат: `com.company.appname`

---

## 🎭 7. ТЕМЫ (СВЕТЛАЯ/ТЕМНАЯ)

### 7.1 Светлая тема
- [ ] **Фон scaffold**
  - Текущий: `Colors.white`
  - Файл: `flutter/lib/common.dart` (строка 379)

- [ ] **Hover цвет**
  - Текущий: `#E0E0E0` (серый)
  - Файл: `flutter/lib/common.dart` (строка 378)

- [ ] **Цвет карточек**
  - Текущий: `MyTheme.grayBg` (#EFEFF2)
  - Файл: `flutter/lib/common.dart` (строка 412)

### 7.2 Темная тема
- [ ] **Фон scaffold**
  - Текущий: `#18191E` (темно-серый)
  - Файл: `flutter/lib/common.dart` (строка 477)

- [ ] **Hover цвет**
  - Текущий: `#2D2E35` (темно-серый)
  - Файл: `flutter/lib/common.dart` (строка 476)

- [ ] **Цвет карточек**
  - Текущий: `#24252B` (темно-серый)
  - Файл: `flutter/lib/common.dart` (строка 514)

---

## 🔧 8. СПЕЦИАЛЬНЫЕ НАСТРОЙКИ

### 8.1 Кастомные шрифты
- [ ] Добавить собственные шрифты
  - Путь: `flutter/assets/fonts/`
  - Настройка: `flutter/pubspec.yaml`

### 8.2 Splash screen / Loading
- [ ] **Экран загрузки**
  - Windows: можно добавить через ресурсы
  - Flutter: `flutter/assets/`

### 8.3 Дополнительные ресурсы
- [ ] **Пользовательские изображения**
  - Путь: `flutter/assets/`
  - Используются в UI

---

## ✅ ЧЕКЛИСТ ФАЙЛОВ ДЛЯ ИЗМЕНЕНИЯ

### Обязательные файлы:
- [ ] `flutter/lib/common.dart` - цвета, темы
- [ ] `flutter/windows/runner/Runner.rc` - метаданные Windows
- [ ] `res/icon.ico` - иконка приложения Windows
- [ ] `res/tray-icon.ico` - иконка трея Windows
- [ ] `res/logo.svg` - векторный логотип
- [ ] `res/*.png` - PNG иконки разных размеров
- [ ] `.github/workflows/flutter-build.yml` - переменные сборки

### Опциональные файлы:
- [ ] `flutter/lib/desktop/pages/desktop_home_page.dart` - главная страница
- [ ] `flutter/lib/desktop/widgets/titlebar_widget.dart` - титульный бар
- [ ] `flutter/assets/logo.png` - растровый логотип
- [ ] `flutter/android/app/src/main/res/values/strings.xml` - Android
- [ ] `flutter/ios/Runner/Info.plist` - iOS метаданные

---

## 📝 ПРИМЕЧАНИЯ

### Цвета в формате hex:
- Формат: `0xAARRGGBB` где:
  - AA = Alpha (прозрачность: FF = непрозрачный, 00 = прозрачный)
  - RR = Red (красный)
  - GG = Green (зеленый)
  - BB = Blue (синий)

### Где применяются цвета:
- `MyTheme.accent` - кнопки, ссылки, активные элементы, прогресс-бары
- `MyTheme.idColor` - отображение ID подключения
- `MyTheme.grayBg` - фон карточек, полей ввода

### Тестирование изменений:
1. Локальная сборка: `flutter build windows`
2. GitHub Actions: push в репозиторий
3. Проверка: установить собранное приложение

---

## 🎯 БЫСТРЫЙ СТАРТ

Минимальный набор для ребрендинга:

1. **Название** (3 места):
   - `Runner.rc` - строки 93, 98
   - `APP_NAME` в workflow

2. **Цвет** (1 место):
   - `common.dart` - строка 254: `MyTheme.accent`

3. **Иконки** (2 файла):
   - `res/icon.ico`
   - `res/tray-icon.ico`

4. **Логотип** (1 файл):
   - `res/logo.svg`

Этого достаточно для базового ребрендинга!
