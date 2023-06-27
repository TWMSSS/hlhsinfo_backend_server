@echo off
set /a lines=0
for /r src %%f in (*.rs) do (
    for /f %%l in ('type "%%f" ^| find /c /v ""') do (
        set /a lines+=%%l
    )
)
echo Total lines of code: %lines%
pause