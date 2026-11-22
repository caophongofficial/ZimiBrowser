# ZimiBrowser

ZimiBrowser là bản Việt hóa và rebrand của **ShardBrowser Launcher**, được build thành phiên bản **offline hoàn toàn**: công cụ trình duyệt, thư viện 220 dấu vân tay và manifest phát hành đều nằm sẵn trong ứng dụng.

> **Nguồn gốc:** dự án này là fork của [ShardBrowser](https://github.com/ProxyShard/ShardBrowser) © 2026 ShardBrowser, phát hành theo giấy phép [MIT](LICENSE). ZimiBrowser giữ nguyên toàn bộ chức năng của bản gốc; chỉ đổi ngôn ngữ, tên/icon và cách phân phối tài nguyên offline. Xem thêm [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md) và [NOTICE](NOTICE).

## Điểm chính

- Giao diện mặc định: **Tiếng Việt**, thứ tự ngôn ngữ `Tiếng Việt → English → Русский → 中文（简体）`.
- Đầy đủ tính năng của ShardBrowser: hồ sơ, proxy, dấu vân tay, tiện ích, dấu trang, tự động hóa, nhóm/đồng bộ, Local API, MCP, Trash.
- Bản offline: không tải engine/fingerprint từ CDN trong quá trình cài đặt và sử dụng bình thường.
- Cập nhật thủ công: chạy bộ cài ZimiBrowser mới; ứng dụng không tự kiểm tra cập nhật nền.
- Thư mục dữ liệu: `%APPDATA%\zimibrowser` trên Windows.
- Không gọi CDN ShardBrowser cho runtime/fingerprint. Các dịch vụ bên ngoài như ShardX vẫn giữ tên gốc vì là dịch vụ độc lập.

## Cài đặt

Tải bản cài hoặc bản portable từ trang [Releases](https://github.com/caophongofficial/ZimiBrowser/releases) của fork.

- Windows: `ZimiBrowser-...-setup.exe` hoặc `ZimiBrowser-...msi`.
- Portable: `ZimiBrowser-portable-win-x64.exe`.

Bản portable cần WebView2 có sẵn trên Windows 10/11. Engine được giải nén lần đầu từ dữ liệu nhúng trong ứng dụng.

## Hướng dẫn sử dụng

- [Hướng dẫn tiếng Việt](docs/HUONG_DAN_NGUOI_DUNG.md)
- [Câu hỏi thường gặp và lỗi đã gặp](docs/CAU_HOI_THUONG_GAP.md): phân biệt bản preview/app thật, mở cửa sổ hồ sơ, proxy free, Web Store proxy, updater, test fingerprint.
- [Release notes bản final](docs/RELEASE_NOTES_v2.1.6.md): danh sách artifact và hash SHA-256.

Tài liệu gốc của ShardBrowser được giữ tại [docs/README.upstream.md](docs/README.upstream.md) để đối chiếu.

## Build từ source

Yêu cầu: Node.js 20+, Rust 1.98+, Visual Studio Build Tools 2022, CMake, NASM và libclang.

Các công cụ build portable được đặt trong `tools/` và đã được gitignore.

Engine Windows được lưu trong `src-tauri/bundled/runtime-parts/ShardX-Windows.zip.001..004` (~206 MB sau khi ghép). GitHub public fork từ chối upload LFS, nên file lớn được chia thành 4 phần nhỏ hơn 100 MB. `src-tauri/build.rs` tự ghép thành `ShardX-Windows.zip` trước khi Rust đọc; không cần tải tay và không cần Git LFS.

```powershell
npm install
npm run check
npm run tauri build
```

Không bật lại CDN cho runtime hoặc fingerprint. Cấu hình updater có endpoints rỗng; cập nhật bằng cách chạy bộ cài mới.

## Ghi công

- Mã nguồn gốc: [ShardBrowser](https://github.com/ProxyShard/ShardBrowser), giấy phép MIT.
- Chromium/engine: giấy phép và thông báo của dự án tương ứng; xem [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).
- ZimiBrowser là bản fork độc lập, không phải sản phẩm chính thức của ShardBrowser, ShardX hoặc ProxyShard.

## Giấy phép

ZimiBrowser giữ giấy phép MIT của dự án gốc. Xem [LICENSE](LICENSE) và [NOTICE](NOTICE).
