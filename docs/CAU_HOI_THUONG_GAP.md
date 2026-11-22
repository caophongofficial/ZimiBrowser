# Câu hỏi thường gặp và lỗi đã từng gặp

Tài liệu này ghi lại các điểm dễ bối rối khi dùng và bảo trì ZimiBrowser 2.1.6. Mục đích là để người mới không phải hỏi lại từ đầu. Các tên kỹ thuật như **Proxy**, **API**, **Cookie**, **WebRTC**, **ShardX** được giữ nguyên.

## 1. Bản xem trước trên server web và bản app thật khác nhau thế nào?

Server preview chạy ở `http://127.0.0.1:1420/` với biến `VITE_MOCK=1`. Đây là bản giả lập để xem giao diện:

- Hồ sơ, proxy, tiện ích, dấu vân tay trong preview lấy từ `src/dev/mock/fixtures.ts`.
- Nút **Bắt đầu / Dừng** trong preview chỉ đổi trạng thái trên màn hình.
- Preview **không mở được cửa sổ trình duyệt thật**.

Muốn mở trình duyệt thật thì chạy bản cài hoặc bản portable ZimiBrowser. Dữ liệu thật nằm ở `%APPDATA%\zimibrowser`.

## 2. Làm sao mở cửa sổ trình duyệt của một hồ sơ?

1. Vào **Hồ sơ**.
2. Tìm hồ sơ cần dùng và bấm **Bắt đầu** ở cuối dòng.
3. Trạng thái chuyển thành **Đang chạy**, cửa sổ trình duyệt riêng của hồ sơ tự mở.

Nếu hồ sơ đang chạy mà bạn muốn đưa cửa sổ lên trước:

- dùng nút **Hiện cửa sổ của …** trên dòng hồ sơ, hoặc
- mở bảng lệnh `Ctrl+K` rồi chọn **Hiển thị cửa sổ**.

Sau khi đổi proxy hoặc bật/tắt tiện ích cho hồ sơ, hãy **Dừng** rồi **Bắt đầu** lại. Cửa sổ cũ vẫn giữ cấu hình cũ cho đến khi restart.

## 3. Nên dùng file cài nào?

| File | Dùng khi nào |
|---|---|
| `ZimiBrowser-2.1.6-x64-setup.exe` | Bộ cài NSIS, khuyến nghị cho máy cá nhân |
| `ZimiBrowser-2.1.6-x64.msi` | Bộ cài Windows Installer, phù hợp máy công ty/quản trị |
| `ZimiBrowser-portable-win-x64.exe` | Không cài đặt, chạy trực tiếp |

Ba bản có cùng nội dung: giao diện tiếng Việt, logo mới, 220 dấu vân tay, engine offline và không tải runtime từ CDN.

Dữ liệu không nằm cạnh file portable mà nằm ở:

```text
%APPDATA%\zimibrowser
```

Bản cài cần WebView2 có sẵn trên Windows 10/11.

Các file cài được đính kèm trên trang Releases, **không commit vào lịch sử Git**: mỗi file nặng 215–264 MB, vượt giới hạn file 100 MB của repository GitHub. Thư mục `release/` trong source đã được gitignore đúng.

## 4. Vì sao bản cài cũ bấm icon không mở, báo lỗi updater?

Bản trung gian trước bản final crash ngay khi khởi động với lỗi:

```text
PluginInitialization("updater", "Error deserializing 'plugins.updater' within your Tauri configuration: invalid type: null")
```

Nguyên nhân: Rust vẫn đăng ký `tauri_plugin_updater`, nhưng file cấu hình không có phần `plugins`, nên Tauri nhận `null`.

Cách sửa trong bản final:

```json
"plugins": {
  "updater": {
    "endpoints": [],
    "pubkey": ""
  }
}
```

Plugin vẫn tồn tại để giữ cấu trúc app, nhưng **endpoints để trống** nên không gọi máy chủ cập nhật nào. Đây là **updater của Tauri**, không phải tiện ích mở rộng của trình duyệt. Logic extension của ShardBrowser vẫn giữ nguyên.

Nếu máy đã cài bản cũ, chỉ cần chạy bộ cài setup mới từ Releases. Trong máy đã thử nghiệm, file `zimibrowser.exe` cũng đã được thay bằng bản final.

## 5. Proxy trong bản test có thật không?

Không. Các proxy trong preview nằm ở `src/dev/mock/fixtures.ts`, ví dụ:

```text
relay-eu.shardx.example
203.0.113.18
198.51.100.7
mock-pass-1
```

Đó là dữ liệu giả:

- `*.example` là domain dành cho tài liệu.
- `192.0.2.x`, `198.51.100.x`, `203.0.113.x` là dải IP tài liệu RFC 5737, không dùng duyệt web thật.
- `mock-pass-*` là mật khẩu giả.

Đừng import mấy dòng đó vào app đã cài rồi bấm Test; chúng sẽ fail. Proxy thật phải do bạn tự thêm hoặc lấy từ dịch vụ ShardX thật. Tên **ShardX** được giữ nguyên vì là dịch vụ bên ngoài, không phải tên sản phẩm ZimiBrowser.

## 6. Proxy free và lỗi `ERR_SOCKS_CONNECTION_FAILED`

Proxy free công khai có thể chết rất nhanh. Trong một lần thử, proxy `195.201.246.53:1101` còn phản hồi, vài phút sau đã đóng kết nối SOCKS. Khi đó Chrome báo:

```text
ERR_SOCKS_CONNECTION_FAILED
This site can't be reached
```

Cách xử lý:

1. Vào **Proxy** và bấm **Test** lại từng dòng ngay trước khi dùng.
2. Chọn proxy còn sống.
3. Đổi proxy cho hồ sơ.
4. **Dừng** rồi **Bắt đầu** lại hồ sơ.
5. Reload tab bị lỗi.

Google vào được mà trang test không vào được không có nghĩa là fingerprint lỗi; thường là proxy đã chết giữa chừng hoặc lúc load trang đó. Proxy free cũng có thể xoay IP, nên kết quả test fingerprint thay đổi giữa các lần là chuyện thường.

Không đăng nhập tài khoản thật, không nhập API key, mật khẩu hoặc mã 2FA qua proxy free. Chỉ dùng để test mở web thông thường.

## 7. Vì sao thêm tiện ích từ Web Store báo lỗi nếu chưa chọn proxy?

Thông báo gốc:

```text
choose a proxy for the Web Store in Settings: the launcher does not reach the store from this computer's IP
```

Đây là thiết kế có chủ đích của bản gốc ShardBrowser, được giữ lại:

- Proxy gắn cho hồ sơ chỉ dùng cho **cửa sổ trình duyệt của hồ sơ đó**.
- Việc tải extension và kiểm tra cập nhật 5 giờ/lần do **launcher** thực hiện.
- Launcher **không bao giờ** dùng IP máy để gọi Chrome Web Store, nhằm tránh liên kết mọi hồ sơ về cùng một IP thật.
- Thư viện extension dùng chung cho nhiều hồ sơ, nên cũng không tự động dùng proxy của một hồ sơ bất kỳ.

Cách sửa:

1. Vào **Cài đặt → Chung**.
2. Tìm **Proxy cho Web Store**.
3. Chọn một proxy còn hoạt động.
4. Vào lại **Tiện ích mở rộng → Thêm tiện ích mở rộng → Thêm từ liên kết**.
5. Dán link Web Store hoặc riêng ID tiện ích.

Sau khi tải, extension chỉ mới có trong thư viện. Vào phần chỉnh sửa/Peek của từng hồ sơ để bật extension đó cho hồ sơ.

Có thể thêm extension từ tệp `.crx`, `.zip` hoặc thư mục giải nén mà không cần Web Store. Extension gốc của ShardBrowser không bị bỏ hoặc thay đổi.

## 8. Nên test dấu vân tay ở đâu?

Các trang đang dùng được và mỗi trang đo một lớp khác nhau:

| Trang | Mục đích chính |
|---|---|
| <https://browserleaks.com/> | IP, WebRTC, Canvas, WebGL, Font, TLS, HTTP/2, Client Hints |
| <https://abrahamjuliot.github.io/creepjs/> | Giá trị “nói dối”, prototype tampering, headless/stealth |
| <https://fingerprint.com/demo/> | Visitor ID kiểu thương mại, confidence, bot/devtools flag |
| <https://pixelscan.net/> | Panel tổng hợp: proxy, leak, consistency, bot |
| <https://www.browserscan.net/> | Authenticity, kernel/UA, WebGPU, DNS/WebRTC, bot |
| <https://iphey.com/> | Trust score, VPN/proxy, DNS leak, blacklist |
| <https://bot.incolumitas.com/> | Bot nâng cao, TCP/IP, TLS/JA3/JA4, hành vi |
| <https://amiunique.org/> | Độ khác biệt so với dataset nghiên cứu |
| <https://coveryourtracks.eff.org/> | Tracker blocking và uniqueness của EFF |
| <https://bot.sannysoft.com/> | Kiểm tra nhanh WebDriver/automation flag |

Cách đọc kết quả:

- Không so sánh điểm số giữa các trang với nhau; mỗi trang một thang đo.
- Khớp múi giờ và ngôn ngữ với quốc gia của IP proxy trước khi test.
- Không mở DevTools/CDP khi test vì có thể bị gắn cờ `developer_tools`.
- Chạy cùng một profile 3 lần, restart app rồi chạy lại 1 lần. Hash Canvas/WebGL phải ổn định.
- Luôn có bản đối chứng là Chrome thường cùng máy, cùng mạng.
- Một trang báo “xanh” không chứng minh sẽ vượt qua hệ thống rủi ro của một website cụ thể.

## 9. Engine offline và vì sao có 4 phần zip

Thư mục nguồn có:

```text
src-tauri/bundled/runtime.json
src-tauri/bundled/ShardX-Fingerprints.zip
src-tauri/bundled/runtime-parts/ShardX-Windows.zip.001..004
```

GitHub public fork từ chối upload LFS (`can not upload new objects to public fork`), nên engine zip ~206 MB được chia thành 4 phần nhỏ hơn 100 MB. Khi build, `src-tauri/build.rs` tự ghép lại thành `ShardX-Windows.zip` trước khi Rust đọc qua `include_bytes!()`.

Nếu clone về thấy file zip đã ghép bị thiếu, không cần tải tay: chỉ cần chạy `npm run tauri build`, build script sẽ tạo lại từ 4 phần. Hash đã được kiểm tra khớp 100%.

Không bật lại CDN cho runtime/fingerprint. Bản final có `endpoints: []` cho updater và dùng tài nguyên nhúng.

## 10. Logo và thương hiệu

Logo đang dùng là mẫu **04 – Trình duyệt an toàn**: biểu tượng cửa sổ trình duyệt kết hợp khiên bảo mật, màu xanh dương → xanh ngọc. Nguồn SVG:

```text
src-tauri/icons/icon-source.svg
```

Cùng thiết kế được áp cho:

- `src-tauri/icons/*` gồm PNG, ICO, ICNS.
- `public/zimi-logo.png`.
- `docs/screenshots/zimi-logo-light.png`, `zimi-logo-dark.png`.
- Component `ZimiLogo`, `ZimiMini` trong `src/shared/icons/index.tsx`.

Tên sản phẩm, tiêu đề cửa sổ, identifier, thư mục dữ liệu và tên artifact đều là ZimiBrowser. Dịch vụ ShardX giữ nguyên tên gốc.

## 11. Thư mục `D:\PHANMEM\ShardBrowser` vẫn còn

Trong máy build có thể còn thư mục `D:\PHANMEM\ShardBrowser` rỗng do Windows giữ handle. Đây không phải source và không chứa tài nguyên dự án. Toàn bộ dự án nằm ở:

```text
D:\PHANMEM\ZimiBrowser
```

Có thể xóa thư mục rỗng sau khi tắt/restart phiên làm việc nếu Windows vẫn giữ.

## 12. Kiểm tra trước khi phát hành

Các lệnh đã chạy thành công cho bản final:

```powershell
npm run check
cargo check --manifest-path src-tauri/Cargo.toml
npm run tauri build
```

`npm run check` gồm 106 unit test, kiểm tra i18n, contrast và colors. `cargo test` biên dịch được nhưng binary test debug trên máy Windows này gặp `STATUS_ENTRYPOINT_NOT_FOUND` ở khâu chạy; đây là lỗi loader môi trường của test binary, không ảnh hưởng bản release.

Chi tiết từng bản cài và hash SHA-256 xem [release notes](RELEASE_NOTES_v2.1.6.md).
