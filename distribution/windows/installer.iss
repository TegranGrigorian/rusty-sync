[Setup]
AppName=RustySync
AppVersion=0.1.0
AppPublisher=TegranGrigorian
AppPublisherURL=https://github.com/TegranGrigorian/rusty-sync
AppSupportURL=https://github.com/TegranGrigorian/rusty-sync/issues
AppUpdatesURL=https://github.com/TegranGrigorian/rusty-sync/releases
DefaultDirName={autopf}\RustySync
DefaultGroupName=RustySync
AllowNoIcons=yes
UninstallDisplayIcon={app}\bin\rusty-sync.exe
Compression=lzma2
SolidCompression=yes
OutputDir=output
OutputBaseFilename=RustySync-Setup-{#SetupSetting("AppVersion")}
SetupIconFile=icon.ico
WizardImageFile=wizard-large.bmp
WizardSmallImageFile=wizard-small.bmp
ArchitecturesInstallIn64BitMode=x64
MinVersion=6.1sp1
LicenseFile=..\..\LICENSE
InfoBeforeFile=INSTALL_INFO.txt

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked
Name: "quicklaunchicon"; Description: "{cm:CreateQuickLaunchIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked; OnlyBelowVersion: 6.1
Name: "addtopath"; Description: "Add RustySync to PATH environment variable"; GroupDescription: "System Integration"

[Files]
; Main executable
Source: "installer\bin\rusty-sync.exe"; DestDir: "{app}\bin"; Flags: ignoreversion

; Python scripts and dependencies
Source: "installer\python\*"; DestDir: "{app}\python"; Flags: ignoreversion recursesubdirs createallsubdirs

; Documentation
Source: "..\..\README.md"; DestDir: "{app}"; DestName: "README.txt"; Flags: ignoreversion isreadme
Source: "..\..\docs\*"; DestDir: "{app}\docs"; Flags: ignoreversion recursesubdirs createallsubdirs skipifsourcedoesntexist

[Icons]
Name: "{group}\RustySync"; Filename: "{app}\bin\rusty-sync.exe"
Name: "{group}\RustySync Documentation"; Filename: "{app}\README.txt"
Name: "{group}\{cm:UninstallProgram,RustySync}"; Filename: "{uninstallexe}"
Name: "{autodesktop}\RustySync"; Filename: "{app}\bin\rusty-sync.exe"; Tasks: desktopicon
Name: "{userappdata}\Microsoft\Internet Explorer\Quick Launch\RustySync"; Filename: "{app}\bin\rusty-sync.exe"; Tasks: quicklaunchicon

[Registry]
; Add to PATH if user selected the option
Root: HKLM; Subkey: "SYSTEM\CurrentControlSet\Control\Session Manager\Environment"; ValueType: expandsz; ValueName: "Path"; ValueData: "{olddata};{app}\bin"; Check: NeedsAddPath('{app}\bin') and IsTaskSelected('addtopath'); Flags: uninsdeletevalue

[Run]
Filename: "{app}\bin\rusty-sync.exe"; Parameters: "--help"; Description: "View RustySync help"; Flags: postinstall skipifsilent nowait
Filename: "{app}\README.txt"; Description: "View README"; Flags: postinstall skipifsilent shellexec unchecked

[UninstallDelete]
Type: filesandordirs; Name: "{app}\python\__pycache__"
Type: files; Name: "{app}\*.log"

[Code]
function NeedsAddPath(Param: string): boolean;
var
  OrigPath: string;
begin
  if not RegQueryStringValue(HKEY_LOCAL_MACHINE,
    'SYSTEM\CurrentControlSet\Control\Session Manager\Environment',
    'Path', OrigPath)
  then begin
    Result := True;
    exit;
  end;
  // Check if path already exists
  Result := Pos(';' + UpperCase(Param) + ';', ';' + UpperCase(OrigPath) + ';') = 0;
end;

procedure CurStepChanged(CurStep: TSetupStep);
begin
  if CurStep = ssPostInstall then
  begin
    // Notify user about Python requirements
    if IsTaskSelected('addtopath') then
    begin
      MsgBox('RustySync has been added to your PATH. You may need to restart your command prompt or terminal to use the rusty-sync command.', mbInformation, MB_OK);
    end;
  end;
end;

function InitializeSetup(): Boolean;
begin
  Result := True;
  // Check for minimum requirements
  if not IsWin64 then
  begin
    MsgBox('RustySync requires a 64-bit version of Windows.', mbError, MB_OK);
    Result := False;
  end;
end;
