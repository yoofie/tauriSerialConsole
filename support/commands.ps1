<# Written by @y00fie (http://yoofie.net) #>

function exportAll([string]$name) {
	$theDate = Get-Date -Format "MM.dd.yyyy - hh.mm.ss tt"
	$export_name = "[" + $theDate + "] $name.zip"

	$compress = @{
		Path             = ".\"
		CompressionLevel = "Fastest"
		DestinationPath  = $export_name
	}
	Compress-Archive @compress -Force
	Write-host "Succesfully exported '$export_name'" -f Green


}

function copy_files([string]$targetLocation) {

	If (Test-Path $targetLocation) {
		
		Copy-Item .\bin\pubApi_c.h $targetLocation
		Copy-Item .\bin\pubApi_cpp.h $targetLocation

		# Create dir if its missing
		If (!(Test-Path "$targetLocation\dbg")) {
			New-Item -Path "$targetLocation" -Name "dbg" -ItemType "directory"
		}

		If ((Test-Path ".\target\debug")) {
			Write-Host "Copying DEBUG DLL + LIB files" -f Green
			Copy-Item .\target\debug\async_engine0.dll "$targetLocation\dbg"
			Copy-Item .\target\debug\async_engine0.lib "$targetLocation\dbg"
		}

		# Create dir if its missing
		If (!(Test-Path "$targetLocation\rel")) {
			New-Item -Path "$targetLocation" -Name "rel" -ItemType "directory"
		}
		
		If ((Test-Path ".\target\release")) {
			Write-Host "Copying RELEASE DLL + LIB files" -f Green
			Copy-Item .\target\release\async_engine0.dll "$targetLocation\rel"
			Copy-Item .\target\release\async_engine0.lib "$targetLocation\rel"
		}
		
	}
 else {
		mkdir bin
	}
	
}


function windowsTerminal() {
	#@%LOCALAPPDATA%\Microsoft\WindowsApps\wt.exe -d %cd%
	Write-host "Launched windows terminal" -f Green	
}

function runDev() {
	$installationPath = .\support\vswhere.exe -prerelease -latest -property installationPath
	Write-host "Visual Studio Location: $installationPath" -f Green
	$vcvars64 = "$installationPath" + "\VC\Auxiliary\Build\vcvars64.bat"
	$vcvars642 = "$installationPath" + "\VC\Auxiliary\Build\"
	$vccmd = "$installationPath" + "\Common7\Tools\LaunchDevCmd.bat"
	Write-host "`t vcvars64 path: $vcvars64" -f Green

	$thePath = replaceSlashes($pwd)
	
	Write-Host "      APPDATA:" + $env:APPDATA
	Write-Host "LOCAL APPDATA:" + $env:LOCALAPPDATA

	$wtt3 = "wt -d $thePath cmd /k `'$vcvars64'"

	Write-Host "The path: $thePath"
	Invoke-Expression "& $wtt3"
}

function stats() {
	If (Test-Path '.\bin') {
		
	}
 else {
		mkdir bin
	}
}

function replaceSlashes([string]$inputVal) {
	$pattern = '\\'
	$result = $inputVal -replace $pattern, '/'
	return $result
}

# TEST FUNCTIONS
function Add-Path($Path) {
	$Path = [Environment]::GetEnvironmentVariable("PATH", "Machine") + [IO.Path]::PathSeparator + $Path
	[Environment]::SetEnvironmentVariable( "Path", $Path, "Machine" )
}

# From ./support folder, run following pecific function like so:
# powershell -command "& { . .\addPath.ps1; printSomething }"
function printSomething() {
	Write-host "PRINT SOMETHING TEST FUNCTION" -f Blue
	Write-host "USER DOMAIN: $Env:UserDomain"
	Write-host "COMPUTER NAME: $Env:ComputerName"

}

# Reference material
# https://stackoverflow.com/questions/1405750/calling-a-specific-powershell-function-from-the-command-line
# https://stackoverflow.com/questions/12850487/invoke-a-second-script-with-arguments-from-a-script
# https://devblogs.microsoft.com/cppblog/finding-the-visual-c-compiler-tools-in-visual-studio-2017/
# # https://superuser.com/questions/1091344/powershell-to-delete-all-files-with-a-certain-file-extension
