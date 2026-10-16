# Archival and non-game software compatibility research (2026-10-15)

This lane searched maintainer repositories and primary project documentation for non-game software reconstructions, discontinued-application compatibility, and reverse-engineered hardware companion software. Search hits were only discovery leads; target, method, platform, and status were taken from the project's own README, protocol notes, release page, or technical reference. First-found dates record this catalog review date. Project start dates remain null when the upstream source did not provide an exact start date.

new-projects.json stages 21 new records. metadata-updates.json stages one source-backed status/summary refinement for the already cataloged Lotus 1-2-3 Linux port. Existing project IDs and exact GitHub URLs were checked against the current SQLite catalog and the discovery manifests present during this audit. No database import, generated-doc update, binary, firmware, screenshot, or other media download was performed.

## What was found

The batch covers Win16 application compatibility (WineVDM); multiple independent ScanSnap and Pakon preservation efforts; modern macOS printing/scanning clients for Canon and Ricoh devices; Linux replacements for old NFC, scanner, pen-tablet, keyboard, stream-deck, display, and network-switch vendor software; and a Linux port of a discontinued network scanner client.

Projects are kept separate where they implement different layers or platforms for the same device:

- holgerkrupp/Scan is a native macOS USB scanner app; appositeit/panelbeater restores the iX1500 touchscreen/panel workflow on Linux; mzyy94/AirScap bridges the iX500 Wi-Fi protocol to eSCL clients.
- wailun42/canon-g3010-chmp-scanner is a small Linux CHMP client; pdrgds/pixma-rs is a separate Rust macOS print/scan bridge.
- The Pakon entries describe distinct implementations: a modern Windows driver (FX35), an F-135+ Linux proof of concept (psix), a Mac app (pakon-macos), an experimental cross-platform client/color engine (pakon-mac), and a Wine/macOS driver API shim that runs the original Kodak client (pakon-tlx-macos).

## Important status and provenance limits

- WineVDM supports Win16 applications through a CPU emulator and Wine-derived API conversion. Its README calls DOS emulation incomplete; DOS programs should not be represented as fully compatible.
- TRCC Linux, Pichau Aqua, Fifine D6, ScanSnap, Panelbeater, AirScap, SP-1130N, Canon, and Pakon records preserve each upstream's distinction between hardware-tested results and inferred model or platform coverage. In particular, the Pichau and SP-1130N hardware matrices are narrow; the Fifine D6 has a documented draw-degradation issue; and ScanSnap iX1500/iX1400/iX1300 profiles are not individually hardware-validated in Scan.
- Huion Note X10 requires a BlueZ protocol workaround. Its note exporter deletes a page after confirming a successful local export unless --keep is passed.
- TP-Link SG3428 includes an automated factory-reset path that erases device configuration; the tested device is SG3428 v2.20 with BootUtil v1.0.0.
- Pakon-macos separates its AGPL code from Kodak firmware, DLLs, and color-profile files, which carry separate terms. pakon-mac is marked WIP and has a license restriction plus OEM resources. pakon-tlx-macos requires external OEM software and firmware but does not ship them. The records avoid presenting these proprietary inputs as project-owned assets.
- The Canon Linux client's repository page did not identify a source license in the material reviewed; that uncertainty is stated in the record. The FX35 driver repository likewise had no license stated on the project page, so reuse terms are not inferred.
- The 123elf patch replaces the broad “remaining terminal-driver work” status with the upstream's exact current behavior. It retains the existing binary-port classification and notes the requirement for the original Lotus UNIX distribution.

## Held and excluded leads

- Microsoft Word for Windows 1.1a source release: the Computer History Museum's historical source release is an original-source publication, not a decompilation or reimplementation project. The separate jmarshall23/msword x64 port was unavailable from direct repository/API checks recorded by the sibling software-apps lane, so it is not staged as a live project.
- Pakon Reference: a valuable reverse-engineered, hardware-verified protocol reference, but it explicitly says it is documentation rather than a driver or scanner application. Used only to cross-check Pakon model and client distinctions.
- PakonClient: the archival reference describes it as a client of the original Windows tlx.dll COM server, not an independent replacement of that application/runtime; held outside this reimplementation batch.
- Standalone original-source releases such as MS-DOS or MacPaint source dumps were not cataloged as decompilation/reimplementation records. A source dump alone does not establish reverse engineering, porting, or behavioral reconstruction.
- Acornsoft VIEW, Lotus 1-2-3/123elf, Microsoft Bob, and other existing catalog rows were not duplicated. Only the Lotus status/summary metadata was refined here.

## Validation

The staged JSON was parsed locally. The project list is checked for unique IDs, unique repository URLs, supported project kinds and target type, first_found: 2026-10-15, and at least two labeled HTTPS evidence links per record. The patches target an existing catalog ID and include their evidence links. The catalog importer was not run, as requested.
