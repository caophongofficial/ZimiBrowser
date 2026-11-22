# Hướng dẫn sử dụng ZimiBrowser

Tài liệu ngắn cho người mới. Các tên như **Proxy**, **API**, **Cookie**, **WebRTC** được giữ nguyên vì là tên kỹ thuật quen thuộc.

## 1. Mở ứng dụng

- Chạy `ZimiBrowser-2.1.6-x64-setup.exe`, `ZimiBrowser-2.1.6-x64.msi`, hoặc tệp `ZimiBrowser-portable-win-x64.exe` nếu dùng bản portable.
- Đây là bản offline: công cụ trình duyệt và 220 dấu vân tay đã nằm sẵn trong bộ cài. Lần đầu mở không cần tải từ mạng.
- Giao diện mặc định là **Tiếng Việt**. Nếu muốn đổi, vào **Cài đặt → Chung → Ngôn ngữ giao diện**.
- Bản offline không tự kiểm tra cập nhật. Muốn cập nhật, hãy chạy bộ cài ZimiBrowser mới. Engine và dấu vân tay được lấy từ bộ cài, không tải từ CDN.
- Dữ liệu nằm ở `%APPDATA%\zimibrowser`, kể cả khi dùng bản portable.

## 2. Thêm Proxy

1. Chọn **Proxy** ở thanh bên.
2. Chọn **Thêm proxy**.
3. Nhập địa chỉ theo dạng `socks5://người_dùng:mật_khẩu@host:cổng` hoặc `http://host:cổng`.
4. Chọn **Kiểm tra** để xem proxy có hoạt động không.

## 3. Tạo hồ sơ

1. Chọn **Hồ sơ → Hồ sơ mới**.
2. Đặt tên dễ nhớ, ví dụ `Shop-01`.
3. Chọn dấu vân tay, múi giờ, ngôn ngữ và các tùy chọn khác nếu cần.
4. Chọn **Lưu**.

Bạn có thể nhân bản hồ sơ để tạo nhiều hồ sơ tương tự nhưng vẫn có dấu vân tay riêng.

## 4. Gán Proxy cho hồ sơ

1. Chọn một hoặc nhiều hồ sơ.
2. Chọn **Gán proxy**.
3. Chọn proxy muốn dùng, rồi xác nhận.
4. Chọn **Bắt đầu** để mở trình duyệt của hồ sơ đó.

Nếu không gán proxy, hồ sơ sẽ dùng kết nối trực tiếp từ máy này.

Khi hồ sơ đang chạy mà bạn muốn đưa cửa sổ lên trước, dùng nút **Hiện cửa sổ của …** trên dòng hồ sơ hoặc mở bảng lệnh `Ctrl+K` rồi chọn **Hiển thị cửa sổ**.

Sau khi đổi proxy hoặc bật/tắt tiện ích, hãy **Dừng** rồi **Bắt đầu** lại hồ sơ để cửa sổ dùng cấu hình mới.

## 5. Sắp xếp hồ sơ

- **Thư mục**: gom hồ sơ theo dự án hoặc khách hàng.
- **Thẻ**: đánh dấu trạng thái, ví dụ `cần 2FA`, `đã ấm`.
- **Ghim**: giữ hồ sơ quan trọng ở đầu danh sách.
- **Dạng xem**: lưu cách lọc và sắp xếp bạn hay dùng.

## 6. Dấu trang và tiện ích mở rộng

- Vào **Dấu trang** để thêm địa chỉ xuất hiện trong mọi hồ sơ của một thư mục.
- Vào **Tiện ích mở rộng** để thêm tiện ích từ liên kết Web Store, tệp `.crx`, `.zip` hoặc thư mục giải nén.
- Chọn tiện ích cho từng hồ sơ trong phần chỉnh sửa hồ sơ.
- Nếu tải từ Web Store bị báo lỗi, vào **Cài đặt → Chung → Proxy cho Web Store** rồi chọn một proxy còn hoạt động. Launcher không dùng IP máy để tải extension; proxy của hồ sơ chỉ dùng cho cửa sổ trình duyệt. Xem phần 7 trong [FAQ](CAU_HOI_THUONG_GAP.md).

## 7. Tự động hóa

- Vào **Tự động hóa** để tạo dự án gồm các bước như mở trang, nhấp chuột, nhập văn bản, chờ, đọc dữ liệu.
- Mỗi bước có nhánh **Hoàn thành** và **Thất bại**, giúp luồng chạy đúng ý.
- Có thể chạy một dự án trên nhiều hồ sơ cùng lúc và xem tiến trình trong cửa sổ đoàn tàu.

## 8. Cài đặt quan trọng

- **API cục bộ**: bật nếu muốn điều khiển ứng dụng từ công cụ khác. Chỉ nên bật khi bạn hiểu rõ.
- **Vị trí dữ liệu**: có thể chuyển hồ sơ, dữ liệu trình duyệt, tiện ích và thùng rác sang ổ khác.
- **Đồng bộ**: dùng máy chủ nhóm tự host nếu muốn mang hồ sơ giữa nhiều máy.
- **Xóa tất cả dữ liệu**: tạo bản sao lưu rồi đưa ứng dụng về trạng thái như mới cài.

## 9. Ngôn ngữ

Thứ tự lựa chọn trong menu: **Tiếng Việt → English → Русский → 中文（简体）**.

Nếu một chuỗi chưa có bản dịch, ứng dụng tự hiển thị bản tiếng Anh tương ứng. Các tên sản phẩm, giao thức và tùy chọn kỹ thuật được giữ nguyên để tránh hiểu nhầm.

## 10. Đọc thêm

- [Câu hỏi thường gặp và lỗi đã gặp](CAU_HOI_THUONG_GAP.md)
- [Release notes bản final với hash SHA-256](RELEASE_NOTES_v2.1.6.md)
- [Tài liệu gốc của ShardBrowser](README.upstream.md)
