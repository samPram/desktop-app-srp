Saran langkah berikutnya (pilih 1 per 1 — saya bantu tulis kode setiap langkah)

Saya rekomendasikan urutan pengerjaan (satu per satu). Untuk tiap langkah saya bisa kirim file kode lengkap & penjelasan.

Sidebar (komponen terpisah) — prioritas: tinggi, kompleksitas: rendah

Pindahkan menu ke src/ui/sidebar.rs.

Tambahkan enum/State untuk menu aktif.

Styling hover/active.

Output: ui/sidebar.rs + perubahan app.rs untuk memanggil modul.

Kenapa selanjutnya: navigasi memudahkan pengujian komponen lain.

Topbar + App Menu — kompleksitas: rendah

Tambah tombol Settings/Run control dan ikon kecil.

Bisa tambahkan file ui/topbar.rs.

Gauges (basic numeric → dial) — kompleksitas: sedang → tinggi

Mulai dengan card numeric (nilai besar) lalu upgrade ke custom-dial menggunakan Painter (menggambar arc + needle).

Saya akan berikan fungsi drawable dial yang mudah disesuaikan (skala, redline, needle anim).

Chart (Horsepower & Torque) — kompleksitas: sedang

Tambahkan egui_plot (atau egui::plot jika tersedia) untuk kurva HP/TQ & legenda.
crates.io
+1

Info Panel (kanan) — sensors & test info — kompleksitas: rendah

Buat komponen ui/info_panel.rs yang mengambil data dari AppState.

Peak Cards (Hp/Tq) — kompleksitas: rendah

ui/cards.rs menampilkan peak values & styling.

Theme & Fonts — kompleksitas: rendah → medium

Tambah theme.rs untuk warna utama, font custom (jika mau), spacing.

Data layer & live feed — kompleksitas: tinggi

Tambah data/dyno_data.rs untuk series, dan hook data dari file/UDP/Serial.

Implement worker thread / channel untuk feed data ke UI (egui thread-safe via ctx.request_repaint).

Packaging & Cross-compile — kompleksitas: medium

Build binary untuk target OS, tambahkan icon, installer.