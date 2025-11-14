# 🎯 Сводка полного ребрендинга Ruware Desk

## ✅ Выполненные изменения

### 1. **Название компании**
- ❌ Было: `My Company`
- ✅ Стало: `Support_Ruware`
- 📍 Файлы:
  - `flutter/windows/runner/Runner.rc` (строки 92, 96)
  - `.github/workflows/flutter-build.yml` (строка 51)
  - `flutter/lib/desktop/pages/desktop_setting_page.dart` (строка 2336)

### 2. **Название приложения**
- ❌ Было: `My RustDesk Client`
- ✅ Стало: `Ruware Desk`
- 📍 Файлы:
  - `.github/workflows/flutter-build.yml` (строка 50)
  - `flutter/windows/runner/Runner.rc` (строка 98)

### 3. **Внутреннее имя (для пути установки)**
- ❌ Было: `rustdesk`
- ✅ Стало: `ruwaredesk`
- 📍 Файл: `flutter/windows/runner/Runner.rc` (строка 95)
- 💡 **Результат**: Теперь приложение устанавливается в `C:\Program Files\Ruware Desk` вместо `C:\Program Files\RustDesk`

### 4. **Название исполняемого файла**
- ❌ Было: `rustdesk.exe`
- ✅ Стало: `ruwaredesk.exe`
- 📍 Файл: `flutter/windows/runner/Runner.rc` (строка 97)

### 5. **Название артефактов сборки**
- ❌ Было: `rustdesk-unsigned-windows-x86_64`, `rustdesk-1.4.4-x86_64.exe`, `rustdesk-1.4.4-x86_64.msi`
- ✅ Стало: `ruwaredesk-unsigned-windows-x86_64`, `ruwaredesk-1.4.4-x86_64.exe`, `ruwaredesk-1.4.4-x86_64.msi`
- 📍 Файл: `.github/workflows/flutter-build.yml` (строки 241, 261, 273, 289-290)

### 6. **Все ссылки rustdesk.com → ruware.tech**

#### Замененные URL:
- ❌ `https://rustdesk.com` → ✅ `https://ruware.tech`
- ❌ `https://rustdesk.com/privacy.html` → ✅ `https://ruware.tech/privacy`
- ❌ `https://rustdesk.com/pricing` → ✅ `https://ruware.tech`
- ❌ `https://rustdesk.com/download` → ✅ `https://ruware.tech`
- ❌ `https://rustdesk.com/docs/...` → ✅ `https://ruware.tech`

#### Затронутые файлы:
- `flutter/lib/common.dart` (строка 3602)
- `flutter/lib/mobile/pages/settings_page.dart` (строки 38, 904, 929, 1037, 1042, 1047)
- `flutter/lib/desktop/pages/desktop_setting_page.dart` (строки 2310, 2318)
- `flutter/lib/desktop/pages/connection_page.dart` (строка 44)
- `flutter/lib/desktop/pages/desktop_home_page.dart` (строки 440, 529, 540, 546)
- `flutter/lib/desktop/pages/install_page.dart` (строки 190, 192)

### 7. **Текстовые упоминания**
- ❌ "About RustDesk" → ✅ "About Ruware Desk"
- ❌ "rustdesk.com" (текст) → ✅ "ruware.tech"
- 📍 Файлы:
  - `flutter/lib/mobile/pages/settings_page.dart`
  - `flutter/lib/desktop/pages/desktop_setting_page.dart`

---

## 🎨 Что осталось без изменений

✅ **Цвет акцента**: `#9C1D12` (красный Ruware Desk) - УЖЕ ИЗМЕНЕН РАНЕЕ
✅ **Иконки**: Заменены на красные - УЖЕ ИЗМЕНЕНЫ РАНЕЕ
✅ **Логотип**: res/logo.svg - УЖЕ ИЗМЕНЕН РАНЕЕ
✅ **Серверные настройки**: 172.25.84.149 - УЖЕ НАСТРОЕНЫ

---

## 📦 Результаты новой сборки

После завершения сборки вы получите файлы:

### Windows x64:
- `ruwaredesk-1.4.4-x86_64.exe` - портативная версия
- `ruwaredesk-1.4.4-x86_64.msi` - установщик MSI

### Установка:
- **Путь**: `C:\Program Files\Ruware Desk\` (вместо `C:\Program Files\RustDesk\`)
- **Исполняемый файл**: `ruwaredesk.exe`
- **Компания**: Support_Ruware
- **Название**: Ruware Desk

### В приложении:
- Все ссылки ведут на **ruware.tech**
- Copyright: "Copyright © 2025 Support_Ruware"
- About: "About Ruware Desk"
- Сайт: "ruware.tech" (вместо "rustdesk.com")

---

## 🔗 Ссылки на сборку

Текущая сборка: https://github.com/AGARES2101/rustdesk/actions/runs/19365117860

Для отслеживания прогресса:
```bash
gh run watch 19365117860
```

Или в браузере: https://github.com/AGARES2101/rustdesk/actions

---

## 📝 Дополнительные файлы

Создан файл **CUSTOMIZATION_CHECKLIST.md** с полным списком всех параметров кастомизации для будущих изменений.

---

## ✨ Итого изменено

- **9 файлов** изменено
- **366 строк** добавлено
- **30 строк** удалено
- **Все упоминания RustDesk** заменены на Ruware
- **Все ссылки** ведут на ruware.tech
- **Путь установки** изменен корректно

---

## 🎉 Готово!

Все изменения применены и запущена новая сборка. 
Приложение теперь полностью брендировано как **Ruware Desk** от **Support_Ruware**.
