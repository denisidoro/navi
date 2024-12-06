
### Initial code from @lurebat (https://github.com/lurebat/)
function Invoke-NaviWidget {
    $ast = $tokens = $errors = $cursor = $null
    [Microsoft.PowerShell.PSConsoleReadLine]::GetBufferState([ref] $ast, [ref] $tokens, [ref] $errors, [ref] $cursor)

    $line = $ast.ToString().Trim()
    $output = $null

    if ([String]::IsNullOrEmpty($line)) {
        $output = navi --print
    }
    else {
        $best_match = (navi --print --best-match --query $line | Out-String).Trim()
        if ([String]::IsNullOrEmpty($best_match)) {
            $output = (navi --print --query "$line" | Out-String).Trim()
        } else {
            $output = $best_match
        }
    }

    [Microsoft.PowerShell.PSConsoleReadLine]::RevertLine()
    [Microsoft.PowerShell.PSConsoleReadLine]::Insert($output)
}

Set-PSReadlineKeyHandler -Key Ctrl+g -ScriptBlock { Invoke-NaviWidget }
