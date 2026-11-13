# B1nLoader

Công cụ sideload IPA cho iOS trên Windows (fork của iloader).

## Yêu cầu

- Windows 10/11 x64
- [Rust](https://rustup.rs) (stable, MSVC)
- [Bun](https://bun.sh) (chỉ cần để chạy Tauri CLI)
- WebView2 (tự tải khi cài)
- iTunes / Apple Mobile Device Support (driver USB của Apple)

## Build

```powershell
bunx @tauri-apps/cli build
```

Hoặc nếu đã cài `tauri-cli`:

```powershell
cargo tauri build
```

Kết quả:

- `target\release\b1nloader.exe`
- `target\release\bundle\nsis\B1nLoader_2.4.1_x64-setup.exe`

`dist\` là frontend đã build sẵn (không kèm source). Không cần chạy bước build frontend.

## Cấu trúc

```
src/            Rust backend (Tauri commands)
src/bin/        pairfix.exe - công cụ sửa pairing thủ công
dist/           Frontend đã build sẵn
icons/          Icon app + installer
capabilities/   Quyền Tauri
windows/        Hook NSIS của installer
tauri.conf.json Cấu hình app/bundle (chỉ Windows, NSIS)
```

## Pairing

Nếu app báo `device does not have pairing file`, chỉ cần kết nối lại và bấm **Trust** trên iPhone.
App tự tạo lại pairing record và tự retry, không cần thao tác thêm.
