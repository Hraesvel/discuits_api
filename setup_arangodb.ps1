$oldPref = $ErrorActionPreference
$ErrorActionPreference = "stop"
try
{
    Get-Command arangosh.exe > $null
    Get-ChildItem .\setup_arangodb_test_env.js > $null
    arangosh --javascript.execute .\setup_arangodb_test_env.js
}
catch [System.Management.Automation.ItemNotFoundException]
{
    echo "Can not find script 'setup_arangodb_test_env.js' in current directory!"
}
catch
{
    echo "Arangodb is required for this program"
}

finally
{
    $ErrorActionPreference = $oldPref
}