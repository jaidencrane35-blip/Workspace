; Workspace NSIS installer hooks — P16.PI1 Slice 1 (Installer Foundation)
; Avoid MessageBox during silent uninstall (user data preserved).

!include "LogicLib.nsh"

!macro NSIS_HOOK_PREINSTALL
  DetailPrint "Workspace: preparing installation (closing running instance if present)"
  ExecWait 'cmd /c taskkill /IM workspace-app.exe /T >nul 2>&1'
  Sleep 400
  ExecWait 'cmd /c taskkill /F /IM workspace-app.exe /T >nul 2>&1'
!macroend

!macro NSIS_HOOK_POSTINSTALL
  DetailPrint "Workspace: writing install manifest for version detection"
  FileOpen $0 "$INSTDIR\install-manifest.json" w
  FileWrite $0 '{$\r$\n'
  FileWrite $0 '  "product": "Workspace",$\r$\n'
  FileWrite $0 '  "identifier": "com.workspace.app",$\r$\n'
  FileWrite $0 '  "version": "${VERSION}",$\r$\n'
  FileWrite $0 '  "installer": "nsis",$\r$\n'
  FileWrite $0 '  "slice": "P16.PI1.S1"$\r$\n'
  FileWrite $0 '}$\r$\n'
  FileClose $0
!macroend

!macro NSIS_HOOK_PREUNINSTALL
  DetailPrint "Workspace: preparing uninstall (closing running instance if present)"
  ExecWait 'cmd /c taskkill /IM workspace-app.exe /T >nul 2>&1'
  Sleep 400
  ExecWait 'cmd /c taskkill /F /IM workspace-app.exe /T >nul 2>&1'
!macroend

!macro NSIS_HOOK_POSTUNINSTALL
  DetailPrint "Workspace: removing install-dir leftovers"
  Delete "$INSTDIR\install-manifest.json"
  RMDir "$INSTDIR"

  ; Preserve Moments / DB by default. Interactive uninstall may purge user data.
  ${IfNot} ${Silent}
    MessageBox MB_YESNO|MB_ICONQUESTION \
      "Remove Workspace user data (Moments, local database, settings)?$\r$\n$\r$\nChoose No to keep data for a future reinstall." \
      IDYES workspace_purge_userdata IDNO workspace_keep_userdata
    workspace_purge_userdata:
      DetailPrint "Workspace: removing user data directories"
      RMDir /r "$APPDATA\com.workspace.app"
      RMDir /r "$LOCALAPPDATA\com.workspace.app"
      Goto workspace_userdata_done
    workspace_keep_userdata:
      DetailPrint "Workspace: user data preserved"
    workspace_userdata_done:
  ${EndIf}
!macroend
