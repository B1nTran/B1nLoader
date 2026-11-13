!macro NSIS_HOOK_PREINSTALL
  ; Check for Microsoft Visual C++ 2015-2022 Redistributable (x64)
  ClearErrors
  ReadRegDWORD $0 HKLM "SOFTWARE\Microsoft\VisualStudio\14.0\VC\Runtimes\x64" "Installed"
  IfErrors 0 +2
    StrCpy $0 0
  StrCmp $0 1 done_vcredist
    MessageBox MB_YESNO|MB_ICONEXCLAMATION "B1nLoader can Microsoft Visual C++ Redistributable (x64) de chay.$\r$\n$\r$\nMay ban chua cai (hoac bi thieu). Ban co muon mo trang tai tu Microsoft bay gio khong?" IDNO done_vcredist
      ExecShell "open" "https://aka.ms/vs/17/release/vc_redist.x64.exe"
      MessageBox MB_OK|MB_ICONINFORMATION "Sau khi cai xong Visual C++ Redistributable, hay mo lai B1nLoader."
  done_vcredist:
!macroend

!macro NSIS_HOOK_PREUNINSTALL
!macroend

!macro NSIS_HOOK_POSTINSTALL
!macroend

!macro NSIS_HOOK_POSTUNINSTALL
!macroend
