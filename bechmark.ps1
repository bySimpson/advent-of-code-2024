
param ($daynr)

cargo build -r -p day_$($daynr)


$Data = Foreach($i in 1..10){
    Invoke-Expression "(Measure-Command { .\target\release\day_$daynr.exe -p .\challenges\challenge_$daynr.txt })"
}

$Data | Measure-Object -Property TotalSeconds -Average -Maximum -Minimum