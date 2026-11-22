Third-party notices
===================

ZimiBrowser là fork của ShardBrowser (MIT). Các thành phần bên dưới có thể có giấy phép riêng.

1. ShardBrowser Launcher
   - Source: https://github.com/ProxyShard/ShardBrowser
   - License: MIT
   - Copyright (c) 2026 ShardBrowser

2. ShardBrowser/Chromium engine (`src-tauri/bundled/ShardX-Windows.zip`)
   - Đây là bản Chromium đã patch do dự án gốc phân phối.
   - Chromium và các thư viện đi kèm chịu giấy phép riêng (BSD-style và các giấy phép third-party).
   - Khi phân phối lại binary engine, cần giữ thông báo bản quyền/giấy phép tương ứng theo yêu cầu của dự án gốc.

3. Thư viện 220 fingerprint (`src-tauri/bundled/ShardX-Fingerprints.zip`)
   - Bộ dữ liệu đi kèm bản phát hành gốc.
   - Người phân phối nên xác nhận điều kiện sử dụng/phiên bản với dự án gốc nếu phát hành công khai.

4. Thư viện npm/Rust và font
   - Xem file LICENSE trong từng package trong `node_modules/`, `ui-kit/node_modules/` và kho Cargo.
   - Font Inter: `src/app/fonts/LICENSE-Inter.txt`.
   - Font Noto Sans: `src-tauri/assets/NotoSans-LICENSE.txt`.

5. ShardX
   - ShardX là dịch vụ proxy độc lập. ZimiBrowser giữ tên ShardX ở các tính năng liên quan đến tài khoản/proxy/API key để phản ánh đúng dịch vụ bên ngoài.

Không dùng logo, tên thương hiệu hoặc tài liệu hình ảnh của ShardBrowser/ShardX để ngụ ý ZimiBrowser là sản phẩm chính thức.
