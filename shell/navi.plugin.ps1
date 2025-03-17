

$null = New-Module {

    function Invoke-Navi {
        $startArgs = @{
            FileName = "navi";
            Arguments = $args;
            RedirectStandardOutput = $true;
            WorkingDirectory = $PWD;
            UseShellExecute = $false;
        }
        $p = [System.Diagnostics.Process]@{StartInfo = $startArgs}

        [void]$p.Start()
        $result = $p.StandardOutput.ReadToEnd()
        $p.WaitForExit()

        $result
    }


    ### Initial code from @lurebat (https://github.com/lurebat/)
    ### See #570 (https://github.com/denisidoro/navi/issues/570) for its original contribution
    function Invoke-NaviWidget {
        $ast = $tokens = $errors = $cursor = $null
        [Microsoft.PowerShell.PSConsoleReadLine]::GetBufferState([ref] $ast, [ref] $tokens, [ref] $errors, [ref] $cursor)

        $line = $ast.ToString().Trim()
        $output = $null

        if ([String]::IsNullOrEmpty($line)) {
            $output = (Invoke-Navi "--print" | Out-String).Trim()
        }
        else {
            $best_match = (Invoke-Navi "--print --best-match --query `"$line`"" | Out-String).Trim()
            if ([String]::IsNullOrEmpty($best_match)) {
                $output = (Invoke-Navi "--print --query `"$line`"" | Out-String).Trim()
            }
            else {
                $output = $best_match
            }
        }

        [Microsoft.PowerShell.PSConsoleReadLine]::RevertLine()
        [Microsoft.PowerShell.PSConsoleReadLine]::InvokePrompt()

        ### Handling the case when the user escapes without selecting any entry
        if (-Not([String]::IsNullOrEmpty($output))) {
            [Microsoft.PowerShell.PSConsoleReadLine]::Insert([String]$output)
        }
    }

    Set-PSReadlineKeyHandler -BriefDescription "A keybinding to open Navi Widget" -Chord Ctrl+g -ScriptBlock { Invoke-NaviWidget }
    Export-ModuleMember -Function @()
}
