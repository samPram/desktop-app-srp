# MotoDyno UI Layout

## Struktur Utama

- **Topbar (Menu Bar)**
- **Sidebar (kiri)**
- **Dashboard Testing (tengah)**
- **Info Panel (kanan)**
- **Bottombar (status bar bawah)**

---

## 1. Topbar (Menu Bar)

- Lokasi: Paling atas window.
- Isi menu:
    - File
    - Settings
    - Runs
    - Help
- Judul aplikasi (tengah): `MotoDyno v1.0.0`

---

## 2. Sidebar (Kiri)

- Lokasi: kiri window, vertikal.
- Warna latar: gelap.
- Isi menu (button list):
    1. Dashboard
    2. Dun Data
    3. Cepols
    4. Run History
    5. Runs
    6. Reports
    7. Seports
    8. Configuration

---

## 3. Dashboard Testing (Tengah)

### 3.1 Gauges

- Lokasi: Bagian atas tengah.
- Komponen:
    - **RPM Gauge**
        - Nilai: `6.85` (x1000/min ≈ 6850 rpm)
        - Skala: 0 → 18
        - Needle berhenti di angka 7.
    - **Speed Gauge**
        - Nilai: `95.2` km/h
        - Skala: 0 → 300
        - Needle berhenti di angka 95.

### 3.2 Chart

- Lokasi: bawah gauges.
- Judul: Grafik Horsepower & Torque.
- Sumbu X: `RPM` (0 – 12,000).
- Sumbu Y: `Horsepower / Torque`.
- Garis:
    - **Horsepower**: merah.
    - **Torque**: biru.
- Legenda: `Horsepower`, `Torque Torque`.

### 3.3 Peak Value Cards

- Lokasi: bawah chart.
- **Peak Horsepower**
    - Value: `125.7 HP`
    - Warna strip: merah.
- **Peak Torque**
    - Value: `98.3 Nm`
    - Warna strip: biru.

---

## 4. Info Panel (Kanan)

### 4.1 Sensors

- Title: `SENNSORS`
- Data:
    - Air-Fuel Ratio: `13FR`
    - AFR: `13.20`
    - Lambda: `0.90`
    - Engine Temp (°C): `98.5`
    - Oil Pressure (°C): `65.0`
    - Intake Temp (°C): `25.3`

### 4.2 Oil Pressure

- Title: `Oil Pressure (psi)`
- Value: `6590.00`

### 4.3 Test Information

- Title: `Test Information`
- Data:
    - Run ID: `#008`
    - Date: `2024-07-26`
    - Time: `Yamaha R1`
    - Max RPM: `00:02:10`

---

## 5. Bottombar (Status Bar)

- Lokasi: paling bawah window.
- Isi:
    - **Kiri**: `Status: Connected to Dyno Hardware. Ready for test.`
    - **Kanan**:
        - `CPU: 15%`
        - `RAM 2.5GB`
        - `Disk Disk 45GB Free`
