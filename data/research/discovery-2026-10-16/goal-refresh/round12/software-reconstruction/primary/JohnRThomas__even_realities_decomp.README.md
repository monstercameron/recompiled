# Decompilation of FW for Even Realities G1 Glasses

An attempt to recreate the FW for ER G1 glasses. This project uses the NRF DFu as a base to start getting the code to compile because the glasses are known to update with that method.

## Progress

### App Core
|         Date | Function Progress | App Progress | Total Functions | Labeled Functions | Excluded Zephyr Functions | App Functions | Labeled App Functions |
| ------------ | ----------------- | ------------ | --------------- | ----------------- | ------------------------- | ------------- | --------------------- |
| *2026-06-30* |            87.01% |       59.82% |            2948 |              2565 |                      2057 |           871 |                521 |
| *2026-06-28* |            86.93% |       59.49% |            2946 |              2561 |                      2057 |           869 |                   517 |
| *2026-06-27* |            86.03% |       58.08% |            2949 |              2537 |                      2057 |           873 |                   507 |
| *2026-06-07* |            87.10% |       57.24% |            2946 |              2566 |                      2062 |           884 |                   506 |
| *2026-05-31* |            86.86% |       56.71% |            2945 |              2558 |                      2058 |           887 |                   503 |
| *2026-05-26* |            86.40% |       56.03% |            2942 |              2542 |                      2046 |           896 |                   502 |
| *2026-05-25* |            85.34% |       54.11% |            2939 |              2508 |                      2015 |           924 |                   500 |
| *2026-05-24* |            84.41% |       52.49% |            2938 |              2480 |                      1995 |           943 |                   495 |
| *2026-05-20* |               84% |          52% |            2885 |              2426 |                      1915 |           970 |                   511 |
| *2026-05-19* |               84% |          52% |            2885 |              2426 |                      1912 |           973 |                   514 |
| *2026-05-13* |               84% |          52% |            2848 |              2395 |                      1889 |           959 |                   506 |
| *2026-05-12* |               83% |          53% |            2847 |              2391 |                      1869 |           978 |                   522 |
| *2026-05-04* |               83% |          52% |            2836 |              2366 |                      1845 |           991 |                   521 |
| *2026-05-03* |               83% |          52% |            2817 |              2352 |                      1843 |           974 |                   509 |
| *2026-05-02* |               83% |              |            2816 |              2351 |                      1889 |               |                       |
| *2026-04-19* |               82% |              |            2805 |              2305 |                      1063 |               |                       |
| *2026-04-05* |               81% |              |            2805 |              2290 |                      1033 |               |                       |

### Net Core
Based on the size of the binary shipped and the size of the binary generated from the DFU OTA sample code, likely this binary has no custom code in it.
|         Date | Function Progress | App Progress | Total Functions | Labeled Functions | Excluded Zephyr Functions | App Functions | Labeled App Functions |
| ------------ | ----------------- | ------------ | --------------- | ----------------- | ------------------------- | ------------- | --------------------- |
| *2026-05-03* |                0% |           0% |            1625 |                 0 |                         0 |             0 |                     0 |

#

# Simultaneous DFU for App and Net Core of nRF5340

This sample demonstrates how to perform Device Firmware Updates (DFU) simultaneously on both the application core and network core of the nRF5340 DK using Bluetooth Low Energy.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Building the Application](#building-the-application)
- [Deploying to Device](#deploying-to-device)
- [Performing DFU Over BLE](#performing-dfu-over-ble)

---

## Prerequisites

Before you start, ensure you have:

- **nRF5340 DK** (Development Kit)
- **nRF Connect SDK v2.5.1** or later installed and configured
- **Toolchain** for nRF Connect SDK v2.5.1 (required compiler and build tools)
- **west tool** available in your PATH
- **nrfjprog** command-line tool installed

> **Important:** This sample has been tested with nRF Connect SDK v2.5.1. Using older or newer versions may cause compatibility issues.

---

## Building the Application

The build process compiles the application for the nRF5340 DK with both cores enabled.

### Step 1: Build the Project

```bash
west build -b nrf5340dk_nrf5340_cpuapp
```

**What this does:**
- Compiles the firmware for both the application core and network core
- Generates the necessary build artifacts including the DFU package
- Places outputs in the `build/` directory

### Step 2: Verify Build Output

After a successful build, you should find:

- `build/zephyr/app_signed.hex` - Signed application firmware
- `build/zephyr/dfu_application.zip` - DFU package (used for OTA updates)

---

## Deploying to Device

This step programs the initial firmware onto the nRF5340 DK and enables the bootloader.

### Flash Initial Firmware

```bash
west flash --recover
```

**What this does:**
- Erases the entire flash memory on the device (`--recover` flag)
- Programs the MCUboot bootloader
- Programs the compiled application firmware for both cores
- Prepares the device for DFU updates

> **Note:** This step is only needed once during initial setup. For subsequent firmware updates, use DFU over BLE.

---

## Performing DFU Over BLE

This process updates the firmware on both cores wirelessly using Bluetooth Low Energy.

### Step 1: Install nRF Connect for Mobile

Download and install the [nRF Connect for Mobile app](https://www.nordicsemi.com/Products/Development-tools/nrf-connect-for-mobile) on your smartphone (iOS or Android).

### Step 2: Prepare the DFU Package

The DFU package was created during the build process at:

```
build/zephyr/dfu_application.zip
```

Transfer this file to your mobile phone using:
- USB cable
- Cloud storage (Google Drive, Dropbox, OneDrive)
- Email or messaging apps

For more details about the DFU package format, see the [nRF Connect SDK documentation](https://developer.nordicsemi.com/nRF_Connect_SDK/doc/2.1.0/nrf/app_build_system.html#output-build-files).

### Step 3: Connect to the Device

1. Open the **nRF Connect for Mobile** app
2. Tap **SCAN** to discover nearby Bluetooth devices
3. Find your nRF5340 DK in the list (usually named something like "nrf5340dk" or similar)
4. Tap **CONNECT** to establish a connection

![App Connect](https://github.com/hellesvik-nordic/samples_for_nrf_connect_sdk/raw/main/.images/nrf_connect_app_connect.png)

### Step 4: Initiate DFU

1. With the device connected, look for the **DFU** button in the app
2. Tap the **DFU** button to start the firmware update process

![App DFU](https://github.com/hellesvik-nordic/samples_for_nrf_connect_sdk/raw/main/.images/nrf_connect_app_dfu.png)

### Step 5: Select and Upload Firmware

1. When prompted, select the `dfu_application.zip` file from your phone
2. Choose **"Confirm Only"** upload mode
3. Tap **Start** to begin the update

**What happens:**
- The app transfers the firmware package to the device over BLE
- The bootloader verifies the firmware integrity
- Both the application core and network core are updated simultaneously

### Step 6: Reset the Device

After the upload completes successfully, reset the device to apply the update:

```bash
nrfjprog --reset
```

Or simply press the **RESET** button on the nRF5340 DK.

**What this does:**
- Restarts the device
- The bootloader processes the new firmware
- Both cores boot with the updated firmware

---

## Summary

| Step | Command/Action | Purpose |
|------|----------------|---------|
| 1 | `west build ...` | Compile firmware for both cores |
| 2 | `west flash --recover` | Program initial firmware to device |
| 3 | Transfer `dfu_application.zip` | Prepare update package for mobile |
| 4 | Connect via nRF Connect app | Establish BLE connection to device |
| 5 | Select file and upload | Transfer new firmware over BLE |
| 6 | `nrfjprog --reset` | Restart device and apply update |

---

## Troubleshooting

- **Build fails:** Ensure your nRF Connect SDK is up-to-date with `west update`
- **Flash fails:** Try unplugging and replugging the nRF5340 DK
- **DFU fails:** Verify the `dfu_application.zip` file is not corrupted
- **Connection drops:** Move closer to the device or reduce interference
