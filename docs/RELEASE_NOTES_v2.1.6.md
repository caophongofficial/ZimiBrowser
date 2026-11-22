# ZimiBrowser 2.1.6 — bản phát hành final offline

Ngày phát hành: 2026-11-22

Đây là bản final dùng để phát hành công khai trên fork <https://github.com/caophongofficial/ZimiBrowser>. Nguồn gốc là [ShardBrowser](https://github.com/ProxyShard/ShardBrowser) theo giấy phép MIT; ZimiBrowser giữ nguyên chức năng gốc, chỉ Việt hóa, đổi thương hiệu và đóng gói tài nguyên offline.

## Tải xuống

Trang phát hành: <https://github.com/caophongofficial/ZimiBrowser/releases/tag/v2.1.6-zimi-final>

| File | Loại | Dung lượng |
|---|---|---:|
| `ZimiBrowser-2.1.6-x64-setup.exe` | Bộ cài NSIS, khuyến nghị cho máy cá nhân | 214.9 MB |
| `ZimiBrowser-2.1.6-x64.msi` | Bộ cài Windows Installer | 221 MB |
| `ZimiBrowser-portable-win-x64.exe` | Chạy trực tiếp, không cài đặt | 264 MB |

Bản portable cần WebView2 có sẵn trên Windows 10/11. Dữ liệu của mọi bản đều nằm ở `%APPDATA%\zimibrowser`.

## SHA-256

```text
ZimiBrowser-2.1.6-x64-setup.exe
6E43F049078FC02FD6DAAD933BA8EFF7550AF604486DC40E5B403FF3ACC90784

ZimiBrowser-2.1.6-x64.msi
28253525D165191480447B69676BDA8D21D7C2F7725B2F2D79EC5A85359E0522

ZimiBrowser-portable-win-x64.exe
C8AD5EB9CA971C2616B68D8CE50D0E0BB913CB406846EC5E6A7D2EDEC23D9B89
```

## Có gì trong bản này

- Giao diện mặc định tiếng Việt, thứ tự ngôn ngữ `Tiếng Việt → English → Русский → 中文（简体）`.
- Đổi tên sản phẩm, identifier `com.zimi.browser`, tiêu đề cửa sổ và thư mục dữ liệu `%APPDATA%\zimibrowser`.
- Áp logo mẫu **04 – Trình duyệt an toàn** cho icon app, About, README và component trong app.
- Đóng gói sẵn engine Chromium `158.0.8127.36`, 220 dấu vân tay và manifest offline. Không tải runtime/fingerprint từ CDN khi cài hoặc dùng bình thường.
- Engine zip được chia thành 4 phần trong source vì GitHub public fork không nhận LFS; `build.rs` tự ghép khi build.
- Sửa lỗi crash lúc khởi động do plugin updater nhận cấu hình `null`. Cấu hình mới để endpoints rỗng nên app không tự cập nhật qua mạng.
- Giữ nguyên toàn bộ logic gốc: hồ sơ, proxy, dấu vân tay, tiện ích mở rộng, dấu trang, tự động hóa, nhóm/đồng bộ, Local API, MCP và Trash.

## Lưu ý khi dùng

- Extension tải từ Chrome Web Store cần chọn **Cài đặt → Chung → Proxy cho Web Store**. Launcher không dùng IP máy cho việc này; đây là thiết kế gốc nhằm tránh gắn kết các hồ sơ về cùng một IP thật.
- Proxy trong trang preview `127.0.0.1:1420` là dữ liệu giả, chỉ để xem giao diện. Không import vào app thật.
- Proxy free có thể chết bất kỳ lúc nào. Bấm **Test** trước khi dùng; nếu trang báo `ERR_SOCKS_CONNECTION_FAILED`, hãy đổi proxy rồi **Dừng/Bắt đầu** lại hồ sơ.
- Không dùng proxy free cho tài khoản thật, API key hoặc mã 2FA.
- Để cập nhật phiên bản mới, chạy bộ cài mới; ứng dụng không tự kiểm tra cập nhật nền.

Giải thích đầy đủ các tình huống trên tại [FAQ và lỗi đã gặp](CAU_HOI_THUONG_GAP.md) và [hướng dẫn sử dụng](HUONG_DAN_NGUOI_DUNG.md).

## Kiểm tra chất lượng

- `npm run check`: pass, 106/106 unit test; i18n, contrast, colors sạch.
- `cargo check`: pass.
- `npm run tauri build`: pass, tạo đủ MSI và NSIS.
- Resource offline: hash 4 phần khớp file engine ghép; 220 fingerprint được nạp đúng lần đầu chạy.
- Metadata file cài: `ProductName = ZimiBrowser`, `ProductVersion = 2.1.6`.
- Đã thử bấm shortcut Desktop sau khi cài: app khởi động, tạo dữ liệu và nạp 220 fingerprint thành công.

`cargo test` biên dịch được nhưng binary test debug trên máy Windows build gặp `STATUS_ENTRYPOINT_NOT_FOUND` ở khâu chạy; đây là vấn đề loader môi trường của test binary, không ảnh hưởng artifact release.
