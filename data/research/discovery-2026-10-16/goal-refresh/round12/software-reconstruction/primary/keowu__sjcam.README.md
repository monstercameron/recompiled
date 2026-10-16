# Lelouch: SJCAM SJ4000 Firmware Research and Custom Firmware

[![CI](https://github.com/keowu/sjcam/actions/workflows/ci.yml/badge.svg)](https://github.com/keowu/sjcam/actions/workflows/ci.yml)

![Lelouch project banner](imgs/intro.png)

[Trocar para Português (Brasil)](README-pt-br.md)

Reverse engineering notes, firmware tooling, proof-of-concept code, and a custom firmware interface for the Allwinner V3 version of the SJCAM SJ4000 Air.

> **Research article**
>
> This repository contains the code and artifacts produced during the research. The full article explains the investigation from the first firmware dump to the Lelouch custom firmware.
>
> [Read the complete research article in English](https://keowu.re/posts/Lelouch--Reverse-Engineering-of-an-SJCAM-Action-Camera-with-proprietary-Android--based-firmware-and-Rewriting-a-completely-OpenSource-CFW)) or [Portuguese](https://keowu.re/posts/Lelouch--Engenharia-Reversa-de-uma-SJCAM-Action-Camera-com-firmware-propriet%C3%A1rio-baseado-no-Android-e-Reescrevendo-um-CFW-completamente-OpenSource).

## About the project

This work started with a simple question: how much control could I get over a cheap action camera without documentation, source code, or vendor support?

The camera turned out to use an Allwinner V3 SoC and a heavily modified Android 4.4 system. Its main application, `sdv`, talks to proprietary camera and display services through Binder, renders the interface with MiniGUI, exposes an HTTP server for firmware updates, and communicates with the SJCAM Zone application through a proprietary protocol.

The repository documents the path from firmware extraction to a working replacement for `sdv`. It includes tools for the firmware formats encountered during the research, IDA databases for the reversed binaries, an AVIOCTRL client, the Lelouch custom firmware source, and the proof of concept associated with CVE-2026-52656.

The code targets the researched Allwinner V3 board with an ST7789V display controller and GC4653 image sensor. Older SJ4000 units based on Novatek NT9665X are included only as part of the earlier research and use a different firmware format. Product names alone are not enough to establish compatibility.

## Lelouch

Lelouch is a replacement interface for the camera's original `sdv` executable. It is built against the proprietary libraries recovered from the firmware and uses the same internal interfaces as the original application.

The current implementation includes:

- Camera preview through the reversed `HerbCamera` and `CedarDisplay` interfaces
- A MiniGUI interface designed for the camera's 320x240 display and physical buttons
- Wi-Fi hotspot control through the Allwinner hardware abstraction layer
- Wi-Fi station mode using a custom ARMv7 build of `wpa_supplicant`
- A small text browser with an on-device keyboard
- Motion detection on preview frames and experimental Haar cascade face detection code
- Battery, time, camera, network, and debug status views
- Binder initialization and communication with the services already present in the firmware

Lelouch is not a complete distribution or a generic firmware for every product sold as an SJ4000. It depends on the libraries, kernel, services, partition layout, display controller, image sensor, and board configuration used by the researched Allwinner V3 unit.

## Repository layout

| Path | Contents |
| --- | --- |
| `Lelouch/` | Custom firmware interface, build environment, recovered headers, proprietary shared libraries, and modified `wpa_supplicant` source |
| `Reversing Scripts/` | Parsers and repackers for Allwinner IMAGEWTY, Allwinner eGON, and Novatek BCL1 firmware |
| `AVIOCTRL Script/ウタ.py` | Interactive client for the proprietary TCP protocol used by SJCAM Zone |
| `Bin Files/` | IDA databases for the main executables and shared libraries analyzed during the research |
| `Firmwares Files/` | Allwinner and Novatek firmware samples used during development |
| `POC/` | CVE-2026-52656 proof of concept, crafted eGON image, and the framebuffer/audio demo payload |

Some directories contain third-party or proprietary material recovered from the device. Their presence is for interoperability and security research and does not make those components open source.

## Building Lelouch

The build was tested on x86-64 Linux. You need Python 3, GNU Make, Git, a C/C++ build environment, and network access for the initial toolchain download.

```bash
git clone https://github.com/keowu/sjcam.git
cd sjcam/Lelouch
python3 prepare_build.py
```

`prepare_build.py` downloads a pinned Android API level 14 sysroot and the ARM `arm-linux-androideabi-4.6` cross-compiler into `Lelouch/android_compilers/`. It then builds the modified static `wpa_supplicant` and runs `make`. The downloaded compiler files and generated binaries are ignored by Git.

The resulting files are:

```text
Lelouch/Lelouch
Lelouch/wpa_supplicant
```

To rebuild after changing the C++ sources:

```bash
cd Lelouch
make clean
make
```

The generated `Lelouch` binary is intended to replace `/bin/sdv` in the extracted root filesystem. Wi-Fi station mode also expects the built `wpa_supplicant` at `/mnt/extsd/sta/wpa_supplicant` on the camera's SD card.

The experimental face detection code looks for the generated cascade file at:

```text
/mnt/extsd/frontalface.casc
```

The repository includes a pre-generated cascade and the converter used to create it:

```bash
cd Lelouch/gen_cascade
python3 gen_cascade.py haarcascade_frontalface_alt2.xml frontalface.casc
```

The complete extraction, root filesystem replacement, repacking, and LiveSuit flashing procedure is documented in the research article linked at the top of this page.

## Firmware tools

All parser entry points are Python 3 scripts with built-in `--help` output and no third-party Python dependencies.

### Allwinner IMAGEWTY

Reads the outer LiveSuit firmware image, extracts its files, and rebuilds an image from an extracted directory.

```bash
python3 "Reversing Scripts/AllWinner IMAGEWTY - ROM Parser/main.py" info firmware.img
python3 "Reversing Scripts/AllWinner IMAGEWTY - ROM Parser/main.py" extract firmware.img -o image_dump
python3 "Reversing Scripts/AllWinner IMAGEWTY - ROM Parser/main.py" repack image_dump rebuilt.img
```

### Allwinner eGON

Inspects, extracts, and repacks the `full_img.fex` container found inside the IMAGEWTY image.

```bash
python3 "Reversing Scripts/AllWinner EGON - Firmware Parser/main.py" info full_img.fex
python3 "Reversing Scripts/AllWinner EGON - Firmware Parser/main.py" extract full_img.fex -o egon_dump
python3 "Reversing Scripts/AllWinner EGON - Firmware Parser/main.py" pack egon_dump \
  --original full_img.fex \
  -o rebuilt_full_img.fex
```

The packer uses the original file as a template so that the original layout and metadata can be preserved while partitions are replaced.

### Novatek BCL1

Decompresses the BCL1 container used by older Novatek-based SJ4000 firmware and can scan the result for embedded assets and firmware sections.

```bash
python3 "Reversing Scripts/Novatek もな BCL1 - Firmware Parser/main.py" \
  "Firmwares Files/OLD_SJCAM_Novatek_NT9665X/SJCAM001.bin" \
  -o firmCode.bin \
  --extract
```

## AVIOCTRL client

`ウタ.py` implements the proprietary TCP protocol used by the mobile application to control the camera on port `6666`. It handles the camera's login sequence, keepalive messages, IOCTRL frames, file transfer, and video stream requests using only the Python standard library.

Connect your computer to the camera's Wi-Fi network and run:

```bash
python3 "AVIOCTRL Script/ウタ.py"
```

The default target is `192.168.100.1:6666`. A different address can be supplied when required:

```bash
python3 "AVIOCTRL Script/ウタ.py" --host 192.168.100.1 --port 6666 --debug
```

The interactive client can read the battery and configuration, inspect the SD card, take photos, start or stop recording, list, upload, download, or delete files, request thumbnails, configure Wi-Fi, request the video stream, and send manual IOCTRL commands.

Formatting the SD card, deleting files, changing Wi-Fi settings, and performing a factory reset modify the camera. Use those commands only on hardware and data you own.

## CVE-2026-52656

The researched firmware exposes an unauthenticated HTTP endpoint while the camera hotspot is active. It accepts arbitrary file uploads and writes them directly to the SD card without validating either the client or the uploaded content. This enables complete firmware replacement because an attacker can upload a custom eGON image named `full_img.fex`; the camera recognizes that filename as an OTA package and installs the attacker-controlled firmware through its normal update flow, as though it were an official update.

Successful exploitation gives the uploaded firmware full control over the software running on the device. The included proof of concept demonstrates this by replacing the original application with a custom MiniGUI/AudioTrack payload that plays the *Bad Apple!!* video using raw RGB565 frames and PCM audio after the malicious OTA image is installed.

The `POC/` directory contains:

- `run_exploit.sh`, which uploads the attacker-controlled `full_img.fex` to the camera
- `full_img.fex`, the crafted eGON firmware image accepted by the camera's OTA update mechanism
- `バイバイ/`, the MiniGUI and AudioTrack *Bad Apple!!* payload used to demonstrate arbitrary code execution after the update
- `extract_rgb_pcm_from_mp4/`, a helper for converting the demonstration video into raw RGB565 and PCM data

The media conversion helper requires `ffmpeg`. The generated `video.rgb` and `audio.pcm` files are expected at the root of the camera's SD card by the demonstration payload.

The proof of concept is destructive by nature. A malformed or incompatible image can permanently brick the camera. Do not run it against devices you do not own or without a complete recovery path.

## Research scope

The project was created for reverse engineering, interoperability, education, and security research on hardware acquired legally by the author. SJCAM, Allwinner, Novatek, Android, and the names of their components belong to their respective owners.

The repository contains original source code alongside firmware samples, reverse engineering databases, headers, and shared libraries originating from third-party products. No license is granted for third-party material. Review the provenance and applicable law before redistributing or using those files outside a research environment.

There is currently no repository-wide open-source license. The `wpa_supplicant` source retains its own upstream license in `Lelouch/modified_lelouch_wpa_supplicant_source/COPYING`.
