@echo off
rd /s /q ldraw
powershell -Command "Invoke-WebRequest https://library.ldraw.org/library/updates/complete.zip -OutFile ldraw.zip"
powershell Expand-Archive ldraw.zip -DestinationPath .
rd ldraw.zip