#define AppName "Xpic"
#define AppExeName AppName + ".exe"
#define AppVersion "0.4.0"
#define OS "windows"
#define ARCH "x86_64"
#define RootDir ".."
#define BinDir RootDir + "\bin"

[Setup]
AppId={{47E862FD-BC4B-4ACB-B55C-54D930469D56}
AppName={#AppName}
AppVersion={#AppVersion}

DefaultDirName={autopf}\{#AppName}
PrivilegesRequired=lowest

OutputBaseFilename=Xpic-{#AppVersion}-{#OS}-{#ARCH}-setup
OutputDir={#BinDir}

SetupIconFile={#RootDir}\crates\xpic-app\assets\app-icon.ico
WizardStyle=modern dynamic
SolidCompression=yes

ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
SetupArchitecture=x64

[Files]
Source: {#BinDir}\{#AppExeName}; DestDir: {app}; Flags: ignoreversion

[Tasks]
Name: desktopicon; Description: {cm:CreateDesktopIcon}; GroupDescription: {cm:AdditionalIcons}

[Icons]
Name: {autoprograms}\{#AppName}; Filename: {app}\{#AppExeName}
Name: {autodesktop}\{#AppName}; Filename: {app}\{#AppExeName}; Tasks: desktopicon

[Run]
Filename: {app}\{#AppExeName}; Description: {cm:LaunchProgram,{#AppName}}; Flags: nowait postinstall skipifsilent

[Languages]
Name: english; MessagesFile: compiler:Default.isl
Name: chinesesimplified; MessagesFile: compiler:Languages\ChineseSimplified.isl
