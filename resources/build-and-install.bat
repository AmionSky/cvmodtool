@echo off 
set toolpath="{tool}"
set config="{config}"

%toolpath% --verbose build --config %config%
if %errorlevel% neq 0 (goto errorhandler)
%toolpath% --verbose package --config %config%
if %errorlevel% neq 0 (goto errorhandler)
%toolpath% --verbose install --config %config%
if %errorlevel% neq 0 (goto errorhandler)
exit /b 0

:errorhandler
pause
exit /b %errorlevel%