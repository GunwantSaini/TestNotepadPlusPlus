; Notepad++ Rust Edition - NSIS Installer Script
; Requires NSIS 3.0 or later: https://nsis.sourceforge.io/

; --------------------------------
; Modern UI Configuration
!include "MUI2.nsh"
!include "FileFunc.nsh"

; --------------------------------
; General Configuration
Name "Notepad++ Rust Edition"
OutFile "notepad-plus-rust-setup.exe"
InstallDir "$PROGRAMFILES64\Notepad++ Rust"
InstallDirRegKey HKLM "Software\NotepadPlusRust" "InstallPath"
RequestExecutionLevel admin

; Version Information
!define VERSION "8.0.0"
!define COMPANY "Notepad++ Rust Contributors"
!define PRODUCT_NAME "Notepad++ Rust Edition"
!define PRODUCT_UNINST_KEY "Software\Microsoft\Windows\CurrentVersion\Uninstall\${PRODUCT_NAME}"
!define PRODUCT_UNINST_ROOT_KEY "HKLM"

; Set compression
SetCompressor /SOLID lzma

; --------------------------------
; Interface Settings
!define MUI_ABORTWARNING
!define MUI_ICON "..\..\assets\icon.ico"
!define MUI_UNICON "..\..\assets\icon.ico"
!define MUI_HEADERIMAGE
!define MUI_HEADERIMAGE_BITMAP "..\..\assets\header.bmp"
!define MUI_WELCOMEFINISHPAGE_BITMAP "..\..\assets\wizard.bmp"

; --------------------------------
; Pages
!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_LICENSE "..\..\LICENSE"
!insertmacro MUI_PAGE_COMPONENTS
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!define MUI_FINISHPAGE_RUN "$INSTDIR\notepad-plus.exe"
!define MUI_FINISHPAGE_RUN_TEXT "Launch Notepad++ Rust Edition"
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES
!insertmacro MUI_UNPAGE_FINISH

; --------------------------------
; Languages
!insertmacro MUI_LANGUAGE "English"

; --------------------------------
; Version Info
VIProductVersion "${VERSION}.0"
VIAddVersionKey "ProductName" "${PRODUCT_NAME}"
VIAddVersionKey "CompanyName" "${COMPANY}"
VIAddVersionKey "FileVersion" "${VERSION}"
VIAddVersionKey "ProductVersion" "${VERSION}"
VIAddVersionKey "FileDescription" "Notepad++ Rust Edition Installer"
VIAddVersionKey "LegalCopyright" "GPL-3.0"

; --------------------------------
; Installer Sections

Section "Notepad++ Rust (required)" SEC_MAIN
  SectionIn RO

  ; Set output path to installation directory
  SetOutPath "$INSTDIR"

  ; Install main executable
  File /r "..\..\target\release\notepad-plus.exe"

  ; Install dependencies (if any DLLs are needed)
  ; File "..\..\target\release\*.dll"

  ; Install documentation
  SetOutPath "$INSTDIR\docs"
  File /nonfatal "..\..\README.md"
  File /nonfatal "..\..\CHANGELOG.md"
  File /nonfatal "..\..\CONTRIBUTING.md"
  File /nonfatal "..\..\LICENSE"

  ; Store installation folder
  WriteRegStr HKLM "Software\NotepadPlusRust" "InstallPath" $INSTDIR
  WriteRegStr HKLM "Software\NotepadPlusRust" "Version" "${VERSION}"

  ; Create uninstaller
  WriteUninstaller "$INSTDIR\Uninstall.exe"

  ; Write uninstall information to registry
  WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "DisplayName" "${PRODUCT_NAME}"
  WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "UninstallString" "$INSTDIR\Uninstall.exe"
  WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "DisplayIcon" "$INSTDIR\notepad-plus.exe"
  WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "DisplayVersion" "${VERSION}"
  WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "Publisher" "${COMPANY}"
  WriteRegDWORD ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "NoModify" 1
  WriteRegDWORD ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "NoRepair" 1

  ; Get install size
  ${GetSize} "$INSTDIR" "/S=0K" $0 $1 $2
  IntFmt $0 "0x%08X" $0
  WriteRegDWORD ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "EstimatedSize" "$0"

SectionEnd

Section "Start Menu Shortcuts" SEC_STARTMENU
  CreateDirectory "$SMPROGRAMS\Notepad++ Rust"
  CreateShortcut "$SMPROGRAMS\Notepad++ Rust\Notepad++ Rust.lnk" "$INSTDIR\notepad-plus.exe"
  CreateShortcut "$SMPROGRAMS\Notepad++ Rust\Uninstall.lnk" "$INSTDIR\Uninstall.exe"
SectionEnd

Section "Desktop Shortcut" SEC_DESKTOP
  CreateShortcut "$DESKTOP\Notepad++ Rust.lnk" "$INSTDIR\notepad-plus.exe"
SectionEnd

Section "Add to PATH" SEC_PATH
  ; Add installation directory to PATH
  EnVar::SetHKLM
  EnVar::AddValue "PATH" "$INSTDIR"
  Pop $0
SectionEnd

Section "File Associations" SEC_ASSOC
  ; Register file associations for common text file types
  WriteRegStr HKCR ".txt" "" "NotepadPlusRust.TextFile"
  WriteRegStr HKCR ".log" "" "NotepadPlusRust.TextFile"
  WriteRegStr HKCR ".md" "" "NotepadPlusRust.TextFile"
  WriteRegStr HKCR ".rs" "" "NotepadPlusRust.TextFile"
  WriteRegStr HKCR ".toml" "" "NotepadPlusRust.TextFile"
  WriteRegStr HKCR ".json" "" "NotepadPlusRust.TextFile"
  WriteRegStr HKCR ".xml" "" "NotepadPlusRust.TextFile"
  WriteRegStr HKCR ".yml" "" "NotepadPlusRust.TextFile"
  WriteRegStr HKCR ".yaml" "" "NotepadPlusRust.TextFile"

  WriteRegStr HKCR "NotepadPlusRust.TextFile" "" "Text File"
  WriteRegStr HKCR "NotepadPlusRust.TextFile\DefaultIcon" "" "$INSTDIR\notepad-plus.exe,0"
  WriteRegStr HKCR "NotepadPlusRust.TextFile\shell\open\command" "" '"$INSTDIR\notepad-plus.exe" "%1"'
  WriteRegStr HKCR "NotepadPlusRust.TextFile\shell\edit\command" "" '"$INSTDIR\notepad-plus.exe" "%1"'

  ; Notify Windows of file association changes
  System::Call 'shell32.dll::SHChangeNotify(i, i, i, i) v (0x08000000, 0, 0, 0)'
SectionEnd

; --------------------------------
; Section Descriptions
!insertmacro MUI_FUNCTION_DESCRIPTION_BEGIN
  !insertmacro MUI_DESCRIPTION_TEXT ${SEC_MAIN} "Core application files (required)"
  !insertmacro MUI_DESCRIPTION_TEXT ${SEC_STARTMENU} "Create Start Menu shortcuts"
  !insertmacro MUI_DESCRIPTION_TEXT ${SEC_DESKTOP} "Create Desktop shortcut"
  !insertmacro MUI_DESCRIPTION_TEXT ${SEC_PATH} "Add Notepad++ Rust to system PATH (allows running from command line)"
  !insertmacro MUI_DESCRIPTION_TEXT ${SEC_ASSOC} "Associate common text file types with Notepad++ Rust"
!insertmacro MUI_FUNCTION_DESCRIPTION_END

; --------------------------------
; Uninstaller Section

Section "Uninstall"
  ; Remove files
  Delete "$INSTDIR\notepad-plus.exe"
  Delete "$INSTDIR\Uninstall.exe"

  ; Remove documentation
  RMDir /r "$INSTDIR\docs"

  ; Remove shortcuts
  Delete "$SMPROGRAMS\Notepad++ Rust\Notepad++ Rust.lnk"
  Delete "$SMPROGRAMS\Notepad++ Rust\Uninstall.lnk"
  RMDir "$SMPROGRAMS\Notepad++ Rust"
  Delete "$DESKTOP\Notepad++ Rust.lnk"

  ; Remove installation directory
  RMDir "$INSTDIR"

  ; Remove registry keys
  DeleteRegKey ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}"
  DeleteRegKey HKLM "Software\NotepadPlusRust"

  ; Remove file associations
  DeleteRegKey HKCR ".txt"
  DeleteRegKey HKCR ".log"
  DeleteRegKey HKCR ".md"
  DeleteRegKey HKCR ".rs"
  DeleteRegKey HKCR ".toml"
  DeleteRegKey HKCR ".json"
  DeleteRegKey HKCR ".xml"
  DeleteRegKey HKCR ".yml"
  DeleteRegKey HKCR ".yaml"
  DeleteRegKey HKCR "NotepadPlusRust.TextFile"

  ; Remove from PATH
  EnVar::SetHKLM
  EnVar::DeleteValue "PATH" "$INSTDIR"
  Pop $0

  ; Notify Windows of file association changes
  System::Call 'shell32.dll::SHChangeNotify(i, i, i, i) v (0x08000000, 0, 0, 0)'

SectionEnd

; --------------------------------
; Installer Functions

Function .onInit
  ; Check if already installed
  ReadRegStr $R0 ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "UninstallString"
  StrCmp $R0 "" done

  MessageBox MB_OKCANCEL|MB_ICONEXCLAMATION \
  "${PRODUCT_NAME} is already installed. $\n$\nClick 'OK' to remove the previous version or 'Cancel' to cancel this upgrade." \
  IDOK uninst
  Abort

uninst:
  ClearErrors
  ExecWait '$R0 _?=$INSTDIR'

done:
FunctionEnd
