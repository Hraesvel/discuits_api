$oldPref = $ErrorActionPreference
$ErrorActionPreference = "stop"
try
{
    Get-Command arangosh.exe > $null

}
catch
{
    "Arangodb is required for this program"
}
finally
{
    $ErrorActionPreference = $oldPref
}